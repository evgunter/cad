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
    Coord, Doc, Evaluation, Formula, Mate, MateSide, Node, NotAFaceName, ProfileProgram,
    RecipeNodeId, Said, Say, SitedFace, Speaker, SpokenNode, TargetAxis, VarKind, member_of, pose,
};
use pncad::select::Resolution;

use crate::session::select::resolves;
use crate::session::{FaceSelection, SessionOp};
use crate::tree::HeldPick;
use crate::vocab::{partial_mirror, vocabulary};

/// **The kinds a mate's face picks are read as**, with their labels: a
/// face read as its plane (with its outward normal), or as its
/// carrier's axis. The kind is the mate's primitive — two planes rest,
/// two axes are coaxial.
///
/// **Deliberately partial**: a face pick reads one face, and only these
/// two kinds are read off one face as a mate's pose. The
/// `partial_mirror!` invocation below forces every [`VarKind`] variant
/// to be offered at a seat of this list or named as deliberately
/// absent, with the reason.
pub const MATE_KINDS: [(VarKind, &str); 2] = [(VarKind::Plane, "plane"), (VarKind::Axis, "axis")];

vocabulary! {
    /// **A freedom a mate's value can set**, as the mate panel offers
    /// it: a slide along the chart's (carried) `x`, `y` or `z`, or the
    /// spin about the target's own direction. Which of them a mate leaves
    /// is its kind's ([`Self::offered_beside`]), and the panel offers those
    /// alone. It mirrors the kernel's [`Coord`] both ways, by two
    /// exhaustive matches ([`Self::coord`], [`Self::of`]), so a freedom
    /// the kernel grows does not build here until the panel offers it or
    /// says why not: a spin about the chart's `x` or `y` is a link of a
    /// chain of turns, which a placement with no direction fixed writes,
    /// never a face mate's.
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
        }
    }

    /// The panel's freedom for the kernel's, `None` for a spin about the
    /// chart's `x` or `y`, which no face mate leaves.
    #[must_use]
    pub fn of(coord: Coord) -> Option<Self> {
        match coord {
            Coord::Slide(TargetAxis::X) => Some(Self::SlideX),
            Coord::Slide(TargetAxis::Y) => Some(Self::SlideY),
            Coord::Slide(TargetAxis::Z) => Some(Self::SlideZ),
            Coord::Spin(None) => Some(Self::Spin),
            Coord::Spin(Some(_)) => None,
        }
    }

    /// **Whether the panel offers freedoms beside a mate of `kind`**: a
    /// plane leaves two of the carried slides and the spin, an axis one
    /// slide and the spin, so each freedom is offered beside either.
    /// Which slides is the copy's geometry's to say (the slide the least
    /// turn carries onto the mated direction is the one a plane fixes and
    /// an axis leaves), so the panel offers each, and the evaluation
    /// refuses slides that do not span what the mate leaves. A kind no
    /// face pick reads ([`MATE_KINDS`]) leaves nothing the panel offers.
    #[must_use]
    pub fn offered_beside(kind: VarKind) -> bool {
        matches!(kind, VarKind::Plane | VarKind::Axis)
    }

    /// **How many slides a mate of `kind` leaves**: two beside a plane,
    /// one beside an axis.
    #[must_use]
    pub fn slides_left_by(kind: VarKind) -> usize {
        kind.symmetry()
            .map_or(0, pncad::document::SubgroupFamily::slides)
    }
}

partial_mirror! {
    VarKind, labelled MATE_KINDS,
    offered [Plane, Axis],
    absent [
        Length => "a number, never a pose",
        Angle => "a number, never a pose",
        Scalar => "a number, never a pose",
        Count => "a number, never a pose",
        Point => "a lone point mate leaves a rotation no value can chart",
        Direction => "read through a plane's normal or an axis's direction, \
                      which a face pick does not offer as a role",
        Frame => "a frame is constructed through an axis and a point, two \
                  reads one face pick does not make",
        Body => "a value, never a pose",
        Bodies => "a value, never a pose",
        Profile => "a value, never a pose",
        Face => "the pick itself, which the pose is read off",
        Edge => "a reference, never a pose",
        Vertex => "a reference, never a pose",
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
    /// The pick does not name a FACE, so no pose is read off it. The
    /// picking door refuses a non-face before a selection exists, so
    /// this answers a `FaceSelection` that reached the tool some other
    /// way — its fields are public and its name is a bare `StableName`
    /// (`work/view/face-selection-carries-a-bare-stable-name`).
    PickIsNotAFace {
        /// Which pick.
        side: MateSide,
        /// The face-name constructor's own refusal, carried.
        refusal: NotAFaceName,
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
    /// The chosen kind is none a face pick reads: a face reads as its
    /// plane or its carrier's axis ([`MATE_KINDS`]).
    KindNotOffered {
        /// The kind asked for.
        kind: VarKind,
    },
    /// A value sets a freedom the mate's kind does not leave, or a slide
    /// past as many as it leaves ([`MateFreedom::offered_beside`],
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
            Self::NotTwoPicks => write!(f, "needs two face picks"),
            Self::PickIsNotAFace { side, refusal } => write!(
                f,
                "the {} pick does not name a face, so no pose is read off it: {refusal}",
                side.name()
            ),
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
            Self::KindNotOffered { kind } => {
                write!(f, "a face pick reads a plane or an axis, not a {}", kind)
            }
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
            | Self::PickIsNotAFace { .. }
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
    One(HeldPick),
    /// Both picks, held; the choice is offered.
    Two {
        /// The `on` pick: a face of the copy the mate places.
        on: HeldPick,
        /// The `to` pick: a face of what the copy joins.
        to: HeldPick,
    },
}

impl MateToolState {
    /// The held picks, `on` then `to`, `None` for a side not yet picked
    /// — what the panel's line says and the viewport marks
    /// ([`HeldPick::face`]).
    pub fn picks(&self) -> [Option<&HeldPick>; 2] {
        match self {
            Self::Idle => [None, None],
            Self::One(on) => [Some(on), None],
            Self::Two { on, to } => [Some(on), Some(to)],
        }
    }
}

/// **The sentence a dropped face pick is said in**: its role, and the
/// pick as its panel line said it ([`HeldPick`]) — the mate tool's drop
/// and the mirror tool's (`crate::combine::MirrorEvent`).
pub(crate) fn face_lost(
    f: &mut core::fmt::Formatter<'_>,
    role: impl core::fmt::Display,
    pick: &HeldPick,
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
        pick: HeldPick,
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
    /// against each other face opposite ways, so a rest carries it.
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
    /// The `on` face: read off the copy's shapes.
    pub on: SitedFace,
    /// The `to` face: read where it was picked.
    pub to: SitedFace,
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

/// **A face read as a pose of `kind`**: its plane, or its carrier's
/// axis.
fn pose_of(kind: VarKind, face: pncad::document::Operand) -> pncad::document::Operand {
    match kind {
        VarKind::Axis => pncad::document::Operand::Pose(Box::new(pose::PoseDef::Axis { of: face })),
        _ => plane_of(face),
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
    face: &SitedFace,
) -> Option<RecipeNodeId> {
    match doc.node(at) {
        Some(Node::Place { .. } | Node::InstantiatePart { .. }) => Some(at),
        _ => member_of(doc, face).map(|member| member.instance),
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

    /// Feed one face pick — the GUI-2 selection vocabulary, consumed
    /// into tool state. The first pick fills `on`, the second `to`; a
    /// third REPLACES `to` (the choice step is still open, and
    /// re-picking the second face is how a person corrects it). The pick
    /// is held as every face pick is ([`HeldPick::new`]).
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, face: FaceSelection) {
        let held = HeldPick::new(doc, face);
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
        let again = |held: &mut HeldPick| *held = held.respoken(doc);
        match &mut self.state {
            MateToolState::Idle => {}
            MateToolState::One(on) => again(on),
            MateToolState::Two { on, to } => {
                again(on);
                again(to);
            }
        }
    }

    /// **The held picks following a takeover**
    /// ([`HeldPick::following`]), their words spoken from `doc`.
    pub fn follow(&mut self, moved: &pncad::document::Moved, doc: &Doc<ProfileProgram>) {
        self.state = match &self.state {
            MateToolState::Idle => MateToolState::Idle,
            MateToolState::One(on) => MateToolState::One(on.following(moved, doc)),
            MateToolState::Two { on, to } => MateToolState::Two {
                on: on.following(moved, doc),
                to: to.following(moved, doc),
            },
        };
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
    fn spoken(doc: &Doc<ProfileProgram>, pick: &HeldPick, node: RecipeNodeId) -> SpokenNode {
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
        let mut lost = |side: MateSide, pick: &HeldPick| -> bool {
            let face = pick.face();
            let verdict = crate::tree::pick_resolution(doc, eval, face.node, face.body, &face.name);
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
    /// the second was picked, each as a pose of the chosen kind, `to`
    /// flipped as chosen, and the values the kind leaves.
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
        let (on, to) = (held_on.face(), held_to.face());
        // The choice's doors FIRST, before any pick is read: the kind
        // among those a face reads, each value against the freedoms the
        // kind leaves.
        if !MATE_KINDS.iter().any(|(kind, _)| *kind == choice.kind) {
            return Err(MateToolError::KindNotOffered { kind: choice.kind });
        }
        let mut slides = 0;
        for (coord, _) in &choice.values {
            if matches!(coord, Coord::Slide(_)) {
                slides += 1;
            }
            if MateFreedom::of(*coord).is_none()
                || !MateFreedom::offered_beside(choice.kind)
                || slides > MateFreedom::slides_left_by(choice.kind)
            {
                return Err(MateToolError::NotAFreedom {
                    coord: *coord,
                    kind: choice.kind,
                });
            }
        }
        let face = |side: MateSide, pick: &FaceSelection, name| {
            pncad::document::FaceName::new(name)
                .map(|name| SitedFace::new(pick.node, name))
                .map_err(|refusal| MateToolError::PickIsNotAFace { side, refusal })
        };
        let picked_on = face(MateSide::On, on, on.name.clone())?;
        let picked_to = face(MateSide::To, to, to.name.clone())?;
        for (side, held) in [(MateSide::On, held_on), (MateSide::To, held_to)] {
            let pick = held.face();
            if doc.node(pick.node).is_none() {
                return Err(MateToolError::PickGone {
                    side,
                    node: Self::spoken(doc, held, pick.node),
                });
            }
        }
        let placement =
            placement_of(doc, on.node, &picked_on).ok_or_else(|| MateToolError::NotACopyPick {
                node: Self::spoken(doc, held_on, on.node),
            })?;
        if placement_of(doc, to.node, &picked_to) == Some(placement) {
            let head = if placement == on.node {
                Self::spoken(doc, held_on, on.node)
            } else {
                doc.spoken(placement)
            };
            return Err(MateToolError::SamePick { head });
        }
        // A `Place`'s copy is read off the shape it copies, by the
        // shape's own name for the face, on the output it copies where
        // the shape's operation defines several (a split's halves).
        let (on_face, on_read) = match doc.node(on.node) {
            Some(Node::Place { .. }) => {
                // A copy carries its body's names, so the face's name
                // is the same on the body it places.
                let (at, port) = crate::world::body_of(doc, on.node, on.body);
                let own = pncad::document::FaceName::new(on.name.clone()).map_err(|refusal| {
                    MateToolError::PickIsNotAFace {
                        side: MateSide::On,
                        refusal,
                    }
                })?;
                let body = match crate::world::seat_read(doc, at, Some(port)) {
                    (at, Some(port)) => pncad::document::Operand::output(at, port),
                    (at, None) => pncad::document::Operand::Node(at),
                };
                let read = pncad::document::Operand::select(body, vec![own.clone().into_name()]);
                (SitedFace::new(at, own), read)
            }
            _ => (picked_on.clone(), picked_on.into()),
        };
        // `to` is read where it was picked: on a copy, the copy itself;
        // on a drawn operation, the output picked ([`crate::session::face_read`]).
        let to_read = match doc.node(to.node) {
            Some(Node::Place { .. } | Node::InstantiatePart { .. }) => picked_to.clone().into(),
            _ => crate::session::face_read(doc, to),
        };
        let target = pose_of(choice.kind, to_read);
        let target = if choice.flip {
            pncad::document::Operand::Pose(Box::new(pose::PoseDef::Flip {
                pose: pose::RoleRead::plain(target),
            }))
        } else {
            target
        };
        Ok(MateProposal {
            placement,
            mate: Mate {
                on: pose_of(choice.kind, on_read),
                to: target,
                // The commit writes the orientation nearest where the
                // copy stands ([`SessionOp::AddMate`]).
                orient: pncad::document::Orientation::IDENTITY,
            },
            on: on_face,
            to: picked_to,
            values: choice.values,
        })
    }
}
