//! **Combining bodies**: the modal layer-3 tools that turn body picks
//! into exactly one committed combining edit, and the lowering their
//! ops go through (GAUTH-4, Phase B).
//!
//! # Shape
//!
//! One shape per tool here, on the shared seat machinery
//! [`crate::seats`] carries: single-select stays ruled (GQ7), so
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
    Said, Say, Speaker, SpokenNode, held_by,
};
use pncad::geom_core::{Tol, Vec3};
use pncad::prelude::StableName;
use pncad::select::SplitHalf;
use pncad::topo::BooleanOp;

use crate::seats::{
    FaceRole, Port, Seat, SeatError, SeatEvent, Seats, pick_said, picks_line, seat_line,
};
use crate::session::refuse::one_body;
use crate::session::{
    BooleanSpec, ClearedSeat, FaceSelection, PartSelectSpec, PatternRuleSpec, SessionOp,
};
use crate::tree::HeldPick;
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
    /// ([`crate::seats::picks_line`]), each pick said by `by` as a seat
    /// says one ([`crate::seats::pick_said`]).
    pub fn line(&self, by: Speaker<'_>) -> String {
        match &self.held {
            BooleanPicks::Members(members) => {
                picks_line(members.iter().enumerate().map(|(i, (node, port))| {
                    (
                        format!("{} {}", Seat::Member.name(), i + 1),
                        Some(pick_said(by, node, *port)),
                    )
                }))
            }
            BooleanPicks::Pair(seats) => seat_line(seats, by),
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
    /// The plane: the face, said on the copy it was clicked on
    /// ([`HeldPick::new`]), with the nodes its words name
    /// as the document last spoke them ([`Self::respeak`]). The line and
    /// the drop both say the plane from it, so a drop after the copy it
    /// was clicked on is deleted says what the line said.
    plane: Option<HeldPick>,
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
        self.plane.as_ref().map(HeldPick::face)
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
        self.plane = Some(HeldPick::new(doc, face));
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
        let face = self.plane.as_ref()?.face();
        let verdict = crate::tree::pick_resolution(doc, eval, face.node, face.body, &face.name);
        if crate::session::select::resolves(&verdict) {
            return None;
        }
        // Said on the body kept at the pick: read again here, a deleted
        // copy would answer its own node, which the line never said.
        Some(MirrorEvent::PlaneLost {
            plane: self.plane.take()?,
            resolution: Box::new(verdict),
        })
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
        if let Some(plane) = &mut self.plane {
            *plane = plane.respoken(doc);
        }
    }

    /// **The plane the tool holds** ([`crate::session::Holds`]).
    pub(crate) fn held(&mut self) -> Vec<crate::session::select::Held<'_>> {
        self.plane
            .iter_mut()
            .map(|held| crate::session::select::Held::One(held))
            .collect()
    }

    /// **The panel's held-picks line**: the body seat and the plane,
    /// each said by `by` — the body seat's node (the body a click on a
    /// copy seats), and the plane as the chrome says a held pick
    /// ([`HeldPick`]), on the copy it was clicked on, since the plane
    /// was taken there. A node `by` does not hold is said as the tool
    /// kept it.
    pub fn line(&self, by: Speaker<'_>) -> String {
        let body = self.seats.said(by, 0);
        let plane = self.plane.as_ref().map(|plane| Said(plane, by).to_string());
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
        let plane = self.plane().cloned().ok_or(SeatError::NoFace {
            role: FaceRole::MirrorPlane,
        })?;
        Ok(SessionOp::AddMirror { body, plane })
    }
}

/// A typed mirror-tool event the chrome renders: the plane's drop. The
/// body seat's drop is a [`SeatEvent`].
#[derive(Debug)]
pub enum MirrorEvent {
    /// The held plane face no longer resolves against the landed
    /// evaluation; the tool dropped it.
    PlaneLost {
        /// The plane that was held, said on the body the panel line said
        /// it on, by the last labels the tool kept for its nodes.
        plane: HeldPick,
        /// The resolution machinery's own verdict.
        resolution: Box<pncad::select::Resolution>,
    },
}

/// A node `by` does not hold is said as the tool kept it.
impl Say for MirrorEvent {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::PlaneLost { plane, .. } => {
                crate::matetool::pick_lost(f, FaceRole::MirrorPlane.name(), plane, by)
            }
        }
    }
}

/// The sentence where no document is at hand: each node as the tool
/// kept it.
impl core::fmt::Display for MirrorEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PlaneLost { plane, .. } => self.say(f, Speaker::held(plane.said())),
        }
    }
}

vocabulary! {
    /// **Which side a clearance requirement bounds** — the clearance
    /// form's choice, and which of [`ClearanceBounds`]' shapes its
    /// commit writes: from below, from above, both at one value, or a
    /// band.
    ///
    /// [`ClearanceBounds`]: pncad::document::ClearanceBounds
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum ClearanceBoundChoice {
        /// `clearance(a, b) ≥ c`.
        #[default]
        AtLeast = "at least",
        /// `clearance(a, b) ≤ c`.
        AtMost = "at most",
        /// `clearance(a, b) = c`.
        Equal = "exactly",
        /// `lo ≤ clearance(a, b) ≤ hi`.
        Between = "between",
    }

    /// Every choice with its button label — the form's radio row.
    pub const ALL;
}

impl ClearanceBoundChoice {
    /// **The bounds this choice writes**: its shape, with every end at
    /// `at`. The insert writes contact (`0`); each end is then a slot
    /// row of the requirement (`SlotId::ClearanceBound`).
    pub fn bounds<S: Clone>(self, at: S) -> pncad::document::ClearanceBounds<S> {
        use pncad::document::ClearanceBounds as B;
        match self {
            Self::AtLeast => B::AtLeast(at),
            Self::AtMost => B::AtMost(at),
            Self::Equal => B::Equal(at),
            Self::Between => B::Within {
                lo: at.clone(),
                hi: at,
            },
        }
    }
}

/// **One seat of a clearance requirement, as the tool holds it**: a
/// body (all of its faces), picked in the tree, or a face, picked in
/// the picture.
///
/// **Held where it is drawn, not where it is built.** A requirement
/// between two parts is about the parts as they stand, so a seat is
/// never re-read on the body a copy places, which every other seated
/// tool does ([`Seats::pick`]); the commit door reads a tree pick of a
/// construction on its world copy (`DocSession`'s `AddClearance` arm).
///
/// That redirect is why the seat is not a [`Seats`]: for a construction
/// the world copies ONCE, a `Seats`-held body would land on the same
/// copy, but a construction copied several times names no one copy —
/// which part the author means is not in the pick — so the tool refuses
/// such a pick itself ([`ClearanceEvent::SeveralCopies`]) and asks for
/// the copy in the picture, where `Seats` would seat the construction
/// and leave the insert door to refuse two seats in different spaces.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ClearancePick {
    /// A body: the node picked, as the document last spoke it.
    Body(SpokenNode),
    /// A face, held as the mirror tool holds its plane.
    Face(HeldPick),
}

/// **The clearance tool**: a requirement's two seats, a body or a face
/// each, committing one [`SessionOp::AddClearance`] with the bounds the
/// form holds.
///
/// A pick fills the first open seat; with both full it replaces the
/// second, the seats' pick rule ([`Seats::pick`]). A face picked in the
/// picture seats that face, on the body the picture draws it on; a node
/// picked in the tree seats its body.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClearanceTool {
    picks: [Option<ClearancePick>; 2],
}

/// **Seat `i`'s role**, as a body and as a face: the first seat, then
/// the second.
fn cleared(i: usize) -> (Seat, FaceRole) {
    if i == 0 {
        (Seat::ClearedFirst, FaceRole::ClearedFirst)
    } else {
        (Seat::ClearedSecond, FaceRole::ClearedSecond)
    }
}

impl ClearanceTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            picks: [None, None],
        }
    }

    /// The seat the next pick lands in.
    fn next(&self) -> usize {
        usize::from(self.picks[0].is_some())
    }

    /// **Feed one node pick**: its body, in the next seat — or, for a
    /// body the world draws more than once, the refusal that names no
    /// one copy ([`ClearanceEvent::SeveralCopies`]), the seat untouched.
    #[must_use = "a declined pick is only reported if its notice is shown"]
    pub fn seat_body(
        &mut self,
        doc: &Doc<ProfileProgram>,
        node: RecipeNodeId,
    ) -> Option<ClearanceEvent> {
        let copies = crate::world::world_copies_of_read(doc, &Operand::Node(node)).len();
        if copies > 1 {
            return Some(ClearanceEvent::SeveralCopies {
                node: doc.spoken(node),
                copies,
            });
        }
        let i = self.next();
        self.picks[i] = Some(ClearancePick::Body(doc.spoken(node)));
        None
    }

    /// **Feed one face pick**: the face, in the next seat.
    pub fn seat_face(&mut self, doc: &Doc<ProfileProgram>, face: FaceSelection) {
        let i = self.next();
        self.picks[i] = Some(ClearancePick::Face(HeldPick::new(doc, face)));
    }

    /// The face seat `i` holds, if it holds one.
    pub fn face(&self, i: usize) -> Option<&FaceSelection> {
        match self.picks.get(i)? {
            Some(ClearancePick::Face(held)) => Some(held.face()),
            Some(ClearancePick::Body(_)) | None => None,
        }
    }

    /// The node seat `i` holds as a body, if it holds one.
    pub fn body(&self, i: usize) -> Option<RecipeNodeId> {
        match self.picks.get(i)? {
            Some(ClearancePick::Body(node)) => Some(node.id()),
            Some(ClearancePick::Face(_)) | None => None,
        }
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.picks = [None, None];
    }

    /// **The body seats' survival step** ([`crate::seats`]): a seat
    /// whose node the document no longer holds is dropped, said by the
    /// last label it had.
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        let mut events = Vec::new();
        for (i, pick) in self.picks.iter_mut().enumerate() {
            let (seat, _) = cleared(i);
            if let Some(ClearancePick::Body(node)) = pick
                && doc.node(node.id()).is_none()
                && let Some(ClearancePick::Body(node)) = pick.take()
            {
                events.push(SeatEvent::PickLost { seat, node });
            }
        }
        events
    }

    /// **The face seats' survival step**: each held face re-read
    /// against the landed pair, and dropped when it no longer resolves
    /// — the mirror tool's plane's rule.
    pub fn reconcile_faces(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
    ) -> Vec<ClearanceEvent> {
        let mut events = Vec::new();
        for (i, pick) in self.picks.iter_mut().enumerate() {
            let (_, role) = cleared(i);
            let Some(ClearancePick::Face(held)) = pick else {
                continue;
            };
            let face = held.face();
            let verdict = crate::tree::pick_resolution(doc, eval, face.node, face.body, &face.name);
            if crate::session::select::resolves(&verdict) {
                continue;
            }
            let Some(ClearancePick::Face(face)) = pick.take() else {
                unreachable!("the seat was just read as a face")
            };
            events.push(ClearanceEvent::FaceLost {
                role,
                face,
                resolution: Box::new(verdict),
            });
        }
        events
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        for pick in self.picks.iter_mut().flatten() {
            match pick {
                ClearancePick::Body(node) => *node = node.respoken(doc),
                ClearancePick::Face(held) => *held = held.respoken(doc),
            }
        }
    }

    /// **The faces the tool holds** ([`crate::session::Holds`]).
    pub(crate) fn held(&mut self) -> Vec<crate::session::select::Held<'_>> {
        self.picks
            .iter_mut()
            .filter_map(|pick| match pick {
                Some(ClearancePick::Face(held)) => Some(crate::session::select::Held::One(held)),
                Some(ClearancePick::Body(_)) | None => None,
            })
            .collect()
    }

    /// **The panel's held-picks line**: each seat said by `by` — a body
    /// as a seat says its node, a face as the chrome says a held pick.
    pub fn line(&self, by: Speaker<'_>) -> String {
        let said = |i: usize| match &self.picks[i] {
            Some(ClearancePick::Body(node)) => Some(pick_said(by, node, Port::Alone)),
            Some(ClearancePick::Face(held)) => Some(Said(held, by).to_string()),
            None => None,
        };
        picks_line([
            (cleared(0).1.name(), said(0)),
            (cleared(1).1.name(), said(1)),
        ])
    }

    /// **The one committed edit**: the requirement between the two
    /// seats, within `bounds`.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] naming the first seat still open.
    pub fn op(
        &self,
        bounds: pncad::document::ClearanceBounds<Formula>,
    ) -> Result<SessionOp, SeatError> {
        let seat = |i: usize| match &self.picks[i] {
            Some(ClearancePick::Body(node)) => Ok(ClearedSeat::Body(Operand::Node(node.id()))),
            Some(ClearancePick::Face(held)) => Ok(ClearedSeat::Face(held.face().clone())),
            None => Err(SeatError::Empty { seat: cleared(i).0 }),
        };
        Ok(SessionOp::AddClearance {
            a: seat(0)?,
            b: seat(1)?,
            bounds,
        })
    }
}

/// A typed clearance-tool event the chrome renders: a face seat's drop.
/// A body seat's drop is a [`SeatEvent`].
#[derive(Debug)]
pub enum ClearanceEvent {
    /// A held face no longer resolves against the landed evaluation;
    /// the tool dropped it and the seat is open.
    FaceLost {
        /// Which seat.
        role: FaceRole,
        /// The face that was held, by the last labels the tool kept.
        face: HeldPick,
        /// The resolution machinery's own verdict.
        resolution: Box<pncad::select::Resolution>,
    },
    /// A tree pick of a body the world draws more than once: the pick
    /// names no one copy, so the tool took nothing.
    SeveralCopies {
        /// The body picked.
        node: SpokenNode,
        /// How many world copies draw it.
        copies: usize,
    },
}

/// A node `by` does not hold is said as the tool kept it.
impl Say for ClearanceEvent {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::FaceLost { role, face, .. } => {
                crate::matetool::pick_lost(f, role.name(), face, by)
            }
            Self::SeveralCopies { node, copies } => {
                let held: HeldNodes = [node.clone()].into_iter().collect();
                write!(
                    f,
                    "{} stands in the world {copies} times, so it names no one part — pick a \
                     face of the copy you mean in the picture",
                    by.or_held(&held).node(node.id())
                )
            }
        }
    }
}

/// The sentence where no document is at hand: each node as the tool
/// kept it.
impl core::fmt::Display for ClearanceEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FaceLost { face, .. } => self.say(f, Speaker::held(face.said())),
            Self::SeveralCopies { node, .. } => {
                let held: HeldNodes = [node.clone()].into_iter().collect();
                self.say(f, Speaker::held(&held))
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

/// **The Transform tool before its door opens**: a pick opens the
/// placement editor in its place ([`crate::matetool::MateTool::moving`]),
/// and a split picked whole waits here for the half that is the body.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TransformTool {
    split: Option<SpokenNode>,
}

impl TransformTool {
    /// A tool waiting for its pick.
    pub const fn new() -> Self {
        Self { split: None }
    }

    /// The split picked whole, waiting for its half.
    pub fn awaiting_half(&self) -> Option<&SpokenNode> {
        self.split.as_ref()
    }

    /// Wait on `split` for its half.
    pub(crate) fn await_half(&mut self, split: SpokenNode) {
        self.split = Some(split);
    }

    /// The split, spoken again from `doc` ([`SpokenNode::respoken`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.split = self.split.take().map(|split| split.respoken(doc));
    }

    /// The survival step: a split `doc` no longer holds stops waiting.
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) {
        if self
            .split
            .as_ref()
            .is_some_and(|split| doc.node(split.id()).is_none())
        {
            self.split = None;
        }
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
    let (mesh, chord) = measured_at_scale(measured, |mesh| mesh.positions.clone())?
        .ok_or_else(|| DuplicateFault::NoExtent { input: spoken() })?;
    // The step is stated in the body's space; the body is measured in
    // its own frame, along that heading read into it.
    let heading = body
        .heading_in_own(Vec3::from_array(STEP_DIRECTION))
        .map_err(|_| DuplicateFault::NoExtent { input: spoken() })?
        .to_array();
    let width = width_along(&mesh.positions, heading)
        .filter(|w| w.is_finite() && *w > 0.0)
        .ok_or_else(|| DuplicateFault::NoExtent { input: spoken() })?;
    Ok(clearance(width, width, chord, tol))
}

/// **Something measured off meshes cut fine for its own size**: `at`
/// at [`crate::scene::SCALE_PROBE_DELTA`] reads its size — the diagonal
/// of the box around the probe's `points` — and `at` again at
/// [`MEASURE_CHORD`] of that size is the measure, answered with the
/// chord it was cut at. `Ok(None)` where the probe has no extent. The
/// one measurement a duplicate's step ([`duplicate_step`]) and an
/// inserted part's frame ([`crate::pickindex::clear_of_drawn`]) take.
///
/// # Errors
///
/// `at`'s.
pub fn measured_at_scale<M, E>(
    mut at: impl FnMut(f64) -> Result<M, E>,
    points: impl Fn(&M) -> Vec<pncad::geom_core::Point3<f64>>,
) -> Result<Option<(M, f64)>, E> {
    let probe = points(&at(crate::scene::SCALE_PROBE_DELTA)?);
    let scale = Vec3::from_array(
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            .map(|axis| width_along(&probe, axis).unwrap_or(0.0)),
    )
    .norm();
    if !(scale.is_finite() && scale > 0.0) {
        return Ok(None);
    }
    let chord = scale * MEASURE_CHORD;
    Ok(Some((at(chord)?, chord)))
}

/// **How far to step a body whose near side is `span` behind the far
/// side of what it must clear**, measured off meshes at `chord`: `span`
/// widened by the meshes' sag on both sides, 2(δ + ε) — the true extent
/// is at most the mesh's plus that ([`duplicate_step`] carries the
/// argument) — then [`DUPLICATE_GAP`] of the stepped body's own `width`,
/// widened the same way, on top. A duplicate clears its original
/// (`span` = `width`); an inserted part clears everything drawn
/// ([`crate::pickindex::clear_of_drawn`]).
pub fn clearance(span: f64, width: f64, chord: f64, tol: Tol) -> f64 {
    let sag = 2.0 * (chord + tol.eps());
    span + sag + (width + sag) * DUPLICATE_GAP
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
/// `None` for a family in no space. Whether the index suits the family
/// it reads is evaluation's question, asked of authored and hand-written
/// documents alike.
pub fn part_node(
    doc: &Doc<ProfileProgram>,
    of: RecipeNodeId,
    select: PartSelectSpec,
) -> Option<AuthoredNode> {
    let PartSelectSpec::Instance(index) = select;
    let target = doc.node_space(of)?;
    Some(Node::place(
        pncad::document::BodyRead::indexed(of, vec![Formula::count(index)]),
        Operand::Var(target),
        pncad::document::stand().bundle(),
    ))
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
