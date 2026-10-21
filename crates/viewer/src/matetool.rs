//! **The mate tool**: a modal layer-3 tool that turns two face picks
//! into one committed mate and the values it names (GUI-4; the plan's
//! ruled addition to G3).
//!
//! # Shape
//!
//! The tool holds **two sequential face picks in tool state** — the
//! GUI-2 single-select vocabulary ([`FaceSelection`]) consumed twice —
//! then a choice of class, kind, flip and values ([`MateChoice`]), then
//! ONE committed action: the mate added to the bundle of the copy it
//! places, and each value added naming the mate's minted id. Everything
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
//! placement copies ([`crate::world::on_body`]); a pick on an instance
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
//! Values set the freedoms the mate's kind leaves (ruling C-2): a plane
//! mate slides along `x` and `y` and spins, an axis mate slides along
//! its axis and spins. A value the kind does not leave is refused here
//! ([`MateToolError::NotAFreedom`]), in the kernel's own reading
//! ([`Coord::admitted_by`]), before any edit exists. A freedom left
//! unset is free.
//!
//! # What the class admits
//!
//! The class choice is exposed through the kernel's own vocabulary
//! ([`admitted_classes`]): the class LIST is [`ContactClass::ALL`] and
//! the verdicts are the [`ClassAdmission`] table — both read, neither
//! restated. `Rest` mints, `Tangent` solves but carries no at-rest
//! record, and everything the vocabulary cannot name is the
//! [`pncad::document::CLASS_DEFERRAL`] deferral, refused typed at
//! [`MateTool::proposal`].
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
    CLASS_DEFERRAL, ClassAdmission, Coord, Doc, Evaluation, Formula, FreedomRefusal, Mate,
    MateSide, Node, NotAFaceName, ProfileProgram, RecipeNodeId, SitedFace, SpokenNode, Target,
    TargetAxis, VarKind, class_admission, member_of, pose,
};
use pncad::select::{ContactClass, Resolution};

use crate::session::select::resolves;
use crate::session::{FaceSelection, SessionOp};
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
    /// it: a slide along the chart's `x`, `y` or `z`, or the spin. Which
    /// of them a mate leaves is its kind's (`Coord::admitted_by`), and
    /// the panel offers those alone. It mirrors the kernel's [`Coord`]
    /// both ways, by two exhaustive matches ([`Self::coord`],
    /// [`Self::of`]), so a freedom the kernel grows does not build here
    /// until the panel offers it or says why not: a spin about the
    /// target's own axis names no mate, and a mate's panel writes values
    /// that name its mate.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MateFreedom {
        /// A slide along the chart's `x`.
        SlideX,
        /// A slide along the chart's `y`.
        SlideY,
        /// A slide along the chart's `z`: the mated direction.
        SlideZ,
        /// The spin about the mated direction.
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
            Self::Spin => Coord::Spin,
        }
    }

    /// The panel's freedom for the kernel's, `None` for a spin about the
    /// target's own axis, which no mate's value sets.
    #[must_use]
    pub fn of(coord: Coord) -> Option<Self> {
        match coord {
            Coord::Slide(TargetAxis::X) => Some(Self::SlideX),
            Coord::Slide(TargetAxis::Y) => Some(Self::SlideY),
            Coord::Slide(TargetAxis::Z) => Some(Self::SlideZ),
            Coord::Spin => Some(Self::Spin),
            Coord::SpinAbout(_) => None,
        }
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
    ],
}

/// One contact class and how far the vocabulary carries it — the
/// admission exposure, verbatim from the kernel's table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MateAdmission {
    /// The class.
    pub class: ContactClass,
    /// The kernel's own verdict on it.
    pub admission: ClassAdmission,
}

/// Every contact class the vocabulary can NAME, with the kernel's
/// admission verdict for each — what the tool's class choice offers.
///
/// Both halves come from the kernel: the class list is
/// [`ContactClass::ALL`] (the enum is `#[non_exhaustive]`, so no list
/// written here could see a variant land) and the verdicts are
/// [`class_admission`]. A class the kernel grows appears here carrying
/// whatever verdict the table gives it.
pub fn admitted_classes() -> Vec<MateAdmission> {
    ContactClass::ALL
        .iter()
        .map(|&class| MateAdmission {
            class,
            admission: class_admission(class),
        })
        .collect()
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
    /// The chosen class is outside the vocabulary
    /// ([`ClassAdmission::NotAdmitted`]): refused HERE, before any
    /// edit exists, with the kernel's own deferral sentence.
    ClassRefused {
        /// The refused class.
        class: ContactClass,
    },
    /// A value sets a freedom the mate's kind does not leave, or leaves
    /// with no chart ([`Coord::admitted_by`]).
    NotAFreedom {
        /// The freedom.
        coord: Coord,
        /// The mate's kind.
        kind: VarKind,
        /// The kernel's reading.
        refusal: FreedomRefusal,
    },
}

impl core::fmt::Display for MateToolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotTwoPicks => write!(f, "the mate tool needs two face picks"),
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
            Self::ClassRefused { class } => {
                write!(
                    f,
                    "class {} is not admitted — {CLASS_DEFERRAL}",
                    class.name()
                )
            }
            Self::NotAFreedom {
                coord,
                kind,
                refusal: FreedomRefusal::NotAFreedom,
            } => write!(f, "a {} mate leaves no {} to set", kind, coord.name()),
            Self::NotAFreedom {
                coord,
                kind,
                refusal: FreedomRefusal::NoChart,
            } => write!(
                f,
                "a {} mate's {} has no chart to set it on",
                kind,
                coord.name()
            ),
        }
    }
}

impl core::error::Error for MateToolError {}

impl MateToolError {
    /// This refusal with its nodes spoken from `doc`, a later version of
    /// the document the tool read ([`crate::session::Refusal::respoken`]):
    /// the panel's line ([`MateToolState::line`]) speaks the session's
    /// document, and a refusal drawn beside it speaks the same one.
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
            | Self::ClassRefused { .. }
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
    One(FaceSelection),
    /// Both picks, held; the choice is offered.
    Two {
        /// The `on` pick: a face of the copy the mate places.
        on: FaceSelection,
        /// The `to` pick: a face of what the copy joins.
        to: FaceSelection,
    },
}

impl MateToolState {
    /// The held picks, `on` then `to`, `None` for a side not yet picked
    /// — what the panel's line says and the viewport marks.
    pub fn picks(&self) -> [Option<&FaceSelection>; 2] {
        match self {
            Self::Idle => [None, None],
            Self::One(on) => [Some(on), None],
            Self::Two { on, to } => [Some(on), Some(to)],
        }
    }

    /// **The line the mate panel shows for its held picks** — the
    /// seated tools' line (`seats::picks_line`), with the mate's two
    /// sides as its roles and each pick said as `face_of` says it, the
    /// drop notice's phrase.
    pub fn line(&self, doc: &Doc<ProfileProgram>) -> String {
        let [on, to] = self.picks();
        crate::seats::picks_line(
            [(MateSide::On, on), (MateSide::To, to)].map(|(side, pick)| {
                (
                    side.name(),
                    pick.map(|pick| face_of(&doc.spoken(pick.node))),
                )
            }),
        )
    }
}

/// **What this tool calls a held pick**: `face of Extrude 000000000003`
/// — the face of the node whose body the pick was taken on, as the
/// document speaks it. The panel item ([`MateToolState::line`]) and the
/// drop notice ([`MateToolEvent`]) both say it.
fn face_of(node: &SpokenNode) -> String {
    format!("face of {node}")
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
        /// The pick that was held.
        pick: FaceSelection,
        /// The pick's node by the last label the tool kept for it
        /// ([`MateTool::respeak`]).
        node: SpokenNode,
        /// The resolution machinery's own verdict, read as the feature
        /// tree reads it ([`crate::tree::resolution_as_drawn`]); boxed
        /// for the same width reason `Standing` boxes it.
        resolution: Box<Resolution>,
    },
}

impl core::fmt::Display for MateToolEvent {
    /// A sentence for the status line — the payload stays typed and
    /// full in the value; this is what a person reads.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { side, node, .. } => write!(
                f,
                "the {} pick (a {}) no longer resolves; the tool dropped it",
                side.name(),
                face_of(node)
            ),
        }
    }
}

/// The person's choice — what the chrome's controls select between
/// picks and commit.
#[derive(Debug, Clone, PartialEq)]
pub struct MateChoice {
    /// The declared contact class.
    pub class: ContactClass,
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
/// sides, the mate and its values, plus the class's admission verdict
/// for the chrome to show beside the commit.
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
    /// Its values, added naming the mate once its id is minted.
    pub values: Vec<(Coord, Formula)>,
    /// The kernel's admission verdict for the mate's class (never
    /// `NotAdmitted` — that refuses at [`MateTool::proposal`]).
    pub admission: ClassAdmission,
}

impl MateProposal {
    /// **The one committed action**: the session op that adds the mate
    /// and its values through the ordinary commit door.
    pub fn op(&self) -> SessionOp {
        SessionOp::AddMate {
            placement: self.placement,
            mate: Box::new(self.mate.clone()),
            values: self.values.clone(),
        }
    }
}

/// **A face read as a pose of `kind`**: its plane, or its carrier's
/// axis.
fn pose_of(kind: VarKind, face: &SitedFace) -> pncad::document::Operand {
    let def = match kind {
        VarKind::Axis => pose::PoseDef::Axis {
            of: face.clone().into(),
        },
        _ => pose::PoseDef::Plane {
            face: face.clone().into(),
        },
    };
    pncad::document::Operand::Pose(Box::new(def))
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
    state: MateToolState,
    /// Each held pick's node as the document last spoke it, `on` then
    /// `to` ([`MateTool::respeak`]) — what a drop notice or a refusal
    /// says once the document no longer holds the node.
    said: [Option<SpokenNode>; 2],
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
    /// re-picking the second face is how a person corrects it).
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, face: FaceSelection) {
        let said = Some(doc.spoken(face.node));
        self.state = match std::mem::take(&mut self.state) {
            MateToolState::Idle => {
                self.said = [said, None];
                MateToolState::One(face)
            }
            MateToolState::One(on) | MateToolState::Two { on, .. } => {
                self.said[1] = said;
                MateToolState::Two { on, to: face }
            }
        };
    }

    /// **The held picks' nodes, spoken again from `doc`**, the shown
    /// document after an operation ([`SpokenNode::respoken`]'s rule): a
    /// node `doc` holds takes its label now, and one it no longer holds
    /// keeps the last it had.
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        crate::seats::respeak_each(&mut self.said, doc);
    }

    /// **The held picks' names re-seated** after an operation that
    /// moved them ([`crate::session::Renamed`]).
    pub fn rename(&mut self, renamed: &crate::session::Renamed) {
        match &mut self.state {
            MateToolState::Idle => {}
            MateToolState::One(on) => renamed.rename(&mut on.name),
            MateToolState::Two { on, to } => {
                renamed.rename(&mut on.name);
                renamed.rename(&mut to.name);
            }
        }
    }

    /// **`side`'s pick's node as a refusal says it**: by the label the
    /// tool kept, re-spoken from the shown document after every
    /// operation; as `doc` holds it only when nothing named was kept.
    fn spoken(&self, doc: &Doc<ProfileProgram>, side: MateSide, node: RecipeNodeId) -> SpokenNode {
        let kept = match side {
            MateSide::On => &self.said[0],
            MateSide::To => &self.said[1],
        };
        match kept {
            Some(kept) if kept.kind().is_some() => kept.clone(),
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
        let [said_on, said_to] = std::mem::take(&mut self.said);
        let mut lost = |side: MateSide, pick: &FaceSelection| -> bool {
            let verdict = crate::tree::pick_resolution(doc, eval, pick.node, pick.body, &pick.name);
            if resolves(&verdict) {
                false
            } else {
                events.push(MateToolEvent::PickLost {
                    side,
                    node: match side {
                        MateSide::On => said_on.clone(),
                        MateSide::To => said_to.clone(),
                    }
                    .unwrap_or_else(|| doc.spoken(pick.node)),
                    pick: pick.clone(),
                    resolution: Box::new(verdict),
                });
                true
            }
        };
        (self.state, self.said) = match std::mem::take(&mut self.state) {
            MateToolState::Idle => (MateToolState::Idle, [None, None]),
            MateToolState::One(on) => {
                if lost(MateSide::On, &on) {
                    (MateToolState::Idle, [None, None])
                } else {
                    (MateToolState::One(on), [said_on, None])
                }
            }
            MateToolState::Two { on, to } => {
                match (lost(MateSide::On, &on), lost(MateSide::To, &to)) {
                    (false, false) => (MateToolState::Two { on, to }, [said_on, said_to]),
                    (false, true) => (MateToolState::One(on), [said_on, None]),
                    (true, false) => (MateToolState::One(to), [said_to, None]),
                    (true, true) => (MateToolState::Idle, [None, None]),
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
        let MateToolState::Two { on, to } = &self.state else {
            return Err(MateToolError::NotTwoPicks);
        };
        // The choice's doors FIRST, before any pick is read: the class
        // in the kernel's own table, the kind among those a face
        // reads, each value against the freedoms the kind leaves.
        let admission = class_admission(choice.class);
        if let ClassAdmission::NotAdmitted = admission {
            return Err(MateToolError::ClassRefused {
                class: choice.class,
            });
        }
        if !MATE_KINDS.iter().any(|(kind, _)| *kind == choice.kind) {
            return Err(MateToolError::KindNotOffered { kind: choice.kind });
        }
        for (coord, _) in &choice.values {
            coord
                .admitted_by(choice.kind)
                .map_err(|refusal| MateToolError::NotAFreedom {
                    coord: *coord,
                    kind: choice.kind,
                    refusal,
                })?;
        }
        let face = |side: MateSide, pick: &FaceSelection, name| {
            pncad::document::FaceName::new(name)
                .map(|name| SitedFace::new(pick.node, name))
                .map_err(|refusal| MateToolError::PickIsNotAFace { side, refusal })
        };
        let picked_on = face(MateSide::On, on, on.name.clone())?;
        let picked_to = face(MateSide::To, to, to.name.clone())?;
        for (side, pick) in [(MateSide::On, on), (MateSide::To, to)] {
            if doc.node(pick.node).is_none() {
                return Err(MateToolError::PickGone {
                    side,
                    node: self.spoken(doc, side, pick.node),
                });
            }
        }
        let placement =
            placement_of(doc, on.node, &picked_on).ok_or_else(|| MateToolError::NotACopyPick {
                node: self.spoken(doc, MateSide::On, on.node),
            })?;
        if placement_of(doc, to.node, &picked_to) == Some(placement) {
            let head = if placement == on.node {
                self.spoken(doc, MateSide::On, on.node)
            } else {
                doc.spoken(placement)
            };
            return Err(MateToolError::SamePick { head });
        }
        // A `Place`'s copy is read off the shape it copies, by the
        // shape's own name for the face.
        let on_face = match doc.node(on.node) {
            Some(Node::Place { .. }) => {
                let (at, _, own) = crate::world::on_body(doc, on.node, on.body, &on.name);
                let own = pncad::document::FaceName::new(own).map_err(|refusal| {
                    MateToolError::PickIsNotAFace {
                        side: MateSide::On,
                        refusal,
                    }
                })?;
                SitedFace::new(at, own)
            }
            _ => picked_on,
        };
        let target = pose_of(choice.kind, &picked_to);
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
                on: pose_of(choice.kind, &on_face),
                to: Target::Pose(target),
                class: choice.class,
            },
            on: on_face,
            to: picked_to,
            values: choice.values,
            admission,
        })
    }
}
