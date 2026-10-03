//! **The mate tool**: a modal layer-3 tool that turns two face picks
//! into exactly one committed mate edit (GUI-4; the plan's ruled
//! addition to G3).
//!
//! # Shape
//!
//! The tool holds **two sequential face picks in tool state** — the
//! GUI-2 single-select vocabulary ([`FaceSelection`]) consumed twice,
//! per the ruling that closed the plan's round-2 OQ-a — then a
//! class/alignment choice from the shipped ASM vocabulary, then ONE
//! committed `DocEdit` adding the mate node. Everything before the
//! commit is tool state: it never enters the document, never enters
//! any history, and dies with the session (G1's transient-state rule).
//!
//! # Where the frames come from
//!
//! A mate's alignment frames are in each member's own part
//! coordinates, and this tool authors each one AS THE PICKED FACE
//! (`MateFrame::FromFace`), which names nothing: the side's frame is
//! its own head's face — the head's name with the walk's
//! qualification stripped (`head_face`), the row of the part's own
//! table the instance placed — so the solve resolves the frame from
//! the face's own canonical pose at every evaluation, in the part's
//! coordinates (`ASSEMBLY.md` A11 rule 5). Nothing is stored twice and no
//! arithmetic happens here: a part edit that moves the face moves the
//! mate with it. What the tool still reads at authoring time is the
//! picked face's pose through `names::interrogate::face_frame`, as a
//! PRE-CHECK that stores nothing — so a face the solve could not
//! resolve (a NURBS carrier with no canonical frame, an N2 tie, a
//! carrier fixing no roll reference) refuses here, typed, in the
//! interrogation door's own words, before any edit exists.
//!
//! # What a pick must be
//!
//! A11's **member vocabulary** is the admission rule, and this tool
//! reads it from the kernel rather than restating it
//! ([`pncad::document::member_of`]). A pick authors the reference the
//! kernel's own walk takes: the name the ray resolved to, read at the
//! node the ray MET — so a pick on a transformed instance says the
//! transformed instance, a pick on a pattern copy authors an
//! `Instance(i)`-headed reference at the pattern, and a pick on a
//! union of placed instances authors the member's face, `FromMember`
//! headed, at the union. Everything the walk cannot stand a member on
//! is
//! [`MateToolError::NotAnInstancePick`]. A copy's frame is read at
//! its MASTER (the member walk takes off one `Instance(i)` per pattern
//! level, `member_reading`):
//! an alignment is in the member's part coordinates, every placed body
//! is a rigid image of the same part, and the composed static offset
//! is the solve's to apply.
//!
//! # What the picked frames admit
//!
//! The class choice is exposed through the kernel's own vocabulary
//! ([`admitted_classes`]): the class LIST is [`ContactClass::ALL`] and
//! the verdicts are the [`ClassAdmission`] table — both read, neither
//! restated, so a class the kernel grows cannot be silently absent
//! here and a verdict cannot drift. Today: `Rest` mints, `Tangent`
//! solves but carries no at-rest record, and everything the
//! vocabulary cannot name — `Fit { g₀ }` first — is the
//! [`pncad::document::CLASS_DEFERRAL`] deferral, refused typed at
//! [`MateTool::proposal`] rather than discovered as a failed node
//! after the edit lands.
//!
//! # Survival
//!
//! Tool state survives a picked reference vanishing (GQ7's recorded
//! constraint, the GUI-2 semantics): [`MateTool::reconcile`] re-reads
//! each held pick against the landed pair and **degrades one step per
//! lost pick**, typed — a lost second pick returns the tool to its
//! one-pick step with the first held; a lost first pick with a live
//! second keeps the second as the held pick. No crash, no silent
//! clear: every drop is a [`MateToolEvent`] the chrome renders.
//!
//! That promotion is right HERE and deliberately DIVERGENT from the
//! seated tools' survival rule ([`crate::seats`]): this tool's picks
//! are an interchangeable pair (`a`/`b` of one mate), so the survivor
//! is still meaningfully "the held pick", where a seated tool's seats
//! are ROLES (a profile against an axis, an operand kept against an
//! operand removed) and promotion would move a node between seats that
//! mean different things. That is also why this tool is the one modal
//! tool NOT built on `Seats`: it shares neither the state (its picks
//! are faces) nor the rule. Both module docs state the divergence so
//! neither reads as an accident of the other.
//!
//! **`reconcile` is the consumer's obligation, and it is forgettable.**
//! The tool is a value beside the session, not inside it, so nothing
//! drives the survival step automatically — the application calls it
//! once per frame (`sync_scene`); a headless consumer that forgets
//! sees nothing until a stale pick refuses at [`MateTool::proposal`],
//! typed but late. The cost of the tool living outside the session is
//! exactly this call; it is stated here so the contract is a sentence
//! a consumer reads rather than a defect they meet.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Alignment, AxisSense, CLASS_DEFERRAL, ClassAdmission, Doc, Evaluation, HeldNodes, MateFrame,
    MatePrimitive, MateSide, Member, NotAFaceName, ProfileProgram, Said, SitedFace, Speaker,
    SpokenNode, class_admission, held_by, member_reading, table_gap,
};
use pncad::prelude::StableName;
use pncad::select::{ContactClass, InterrogateError, Resolution, RunCtx, face_frame, resolve};

use crate::session::select::resolves;
use crate::session::{FaceSelection, SessionOp};

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
/// Both halves come from the kernel: the CLASS LIST is
/// [`ContactClass::ALL`] (the enum is `#[non_exhaustive]`, so no
/// enumeration written here could see a variant land — a hand-kept
/// list compiled clean and silently omitted a planted one), and the
/// VERDICTS are [`class_admission`]. A class the kernel grows appears
/// here automatically, carrying whatever verdict the table gives it —
/// `NotAdmitted` by that table's own wildcard until it is admitted
/// deliberately.
///
/// `Fit { g₀ }` does not appear because the kernel enum has no such
/// variant to name: its absence IS the v1 deferral, and the sentence
/// for it is [`CLASS_DEFERRAL`], which [`MateToolError::ClassRefused`]
/// carries for any class the table answers
/// [`ClassAdmission::NotAdmitted`] on.
pub fn admitted_classes() -> Vec<MateAdmission> {
    ContactClass::ALL
        .iter()
        .map(|&class| MateAdmission {
            class,
            admission: class_admission(class),
        })
        .collect()
}

/// **The reference a pick authors, the member it resolves to, and the
/// name its face reads at**: the pick's own operand — the node the ray
/// met — paired with the picked name, the kernel's member for that
/// pair, and the name the member walk reached at the member's instance
/// ([`pncad::document::member_reading`]; a copy reaches its MASTER's
/// name).
///
/// The admission rule is A11's member vocabulary READ, not restated
/// ([`pncad::document::member_of`]): the walk from the operand down to
/// the name's head, through transforms, `Part` instance selections,
/// any number of pattern levels and any number of unions (at the
/// member the name says), ending on a live `InstantiatePart`. The walk
/// refuses a pair boolean's node because a boolean is not a
/// pass-through: it mints its own geometry and its own names, and no
/// member stands on it.
///
/// # Errors
///
/// [`MateToolError::PickIsNotAFace`] when the selection does not name
/// a face, and [`MateToolError::NotAnInstancePick`] for everything
/// outside the member vocabulary.
fn picked_member(
    doc: &Doc<ProfileProgram>,
    side: MateSide,
    pick: &FaceSelection,
) -> Result<(SitedFace, Member, StableName), MateToolError> {
    // A head is a `FaceName`, and this is the boundary that makes one
    // out of a selection. The picking door refuses
    // `SelectionRefusal::NotAFace` before a selection exists, so a
    // caller that went through it never meets this refusal — but
    // `FaceSelection`'s fields are public and its name is a bare
    // `StableName`, so the rule is the DOOR's and not the value's, and
    // a value that arrives another way is refused in every build
    // rather than asserted against in one. The refusal is the
    // constructor's own sentence, carried: what this tool knows about
    // the mistake is exactly what the constructor said.
    let name = editor_core::FaceName::new(pick.name.clone())
        .map_err(|refusal| MateToolError::PickIsNotAFace { side, refusal })?;
    // The pick's own operand: the node the ray met, which is the node
    // whose body was drawn and therefore the geometry the author is
    // pointing at.
    let reference = SitedFace::new(pick.node, name);
    let (member, placed) =
        member_reading(doc, &reference).ok_or_else(|| MateToolError::NotAnInstancePick {
            side,
            node: doc.spoken(pick.node),
        })?;
    let placed = placed.clone();
    Ok((reference, member, placed))
}

/// A typed mate-tool refusal (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum MateToolError {
    /// The tool does not hold two picks yet.
    NotTwoPicks,
    /// The pick does not name a FACE, so there is no mate head to
    /// make out of it: a mate is a face-pair contact and a head is a
    /// `FaceName`. The picking door refuses a non-face before a
    /// selection exists, so this answers a `FaceSelection` that
    /// reached the tool some other way — its fields are public and
    /// its name is a bare `StableName`, so the rule it carries is its
    /// door's rather than its type's
    /// (`work/view/face-selection-carries-a-bare-stable-name`: the
    /// name becomes a `FaceName` and this arm goes away with it).
    PickIsNotAFace {
        /// Which pick.
        side: MateSide,
        /// The head constructor's own refusal, carried rather than
        /// restated.
        refusal: NotAFaceName,
    },
    /// The pick's reference is outside A11's member vocabulary, so
    /// there is no member to mate: the walk from the node the ray met
    /// down to the name's head runs through something that is not a
    /// transform, a `Part` instance selection, or a pattern level or
    /// union the name qualifies, or ends on something that is not a live
    /// `InstantiatePart`
    /// ([`pncad::document::member_of`]) — a pair boolean's body, a
    /// split half.
    NotAnInstancePick {
        /// Which pick.
        side: MateSide,
        /// The node the pick's body belongs to, as the document held it.
        node: SpokenNode,
    },
    /// Both picks name ONE member of A11's vocabulary. A mate relates
    /// a pair; the tool refuses here rather than authoring the edit
    /// the solve would refuse as a self-mate. Two COPIES of one
    /// pattern are two members and are not this refusal.
    SamePick {
        /// The head node both picks name, as the document held it.
        head: SpokenNode,
    },
    /// A picked face's frame could not be derived — the interrogation
    /// door's own refusal (an unresolved name, an N2 tie, a NURBS
    /// face with no canonical frame), with a node that has no value
    /// named as the feature tree names it
    /// ([`crate::tree::interrogation_as_drawn`]).
    ///
    /// Its `through` may be a mate, which is not the DAG ancestor
    /// `NodeStanding` documents
    /// (`work/wire/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`).
    Frame {
        /// Which pick.
        side: MateSide,
        /// The door's refusal.
        error: InterrogateError,
        /// The nodes `error` names, as the document held them
        /// ([`held_by`]).
        held: HeldNodes,
    },
    /// The chosen class is outside the vocabulary
    /// ([`ClassAdmission::NotAdmitted`]): refused HERE, before any
    /// edit exists, with the kernel's own deferral sentence.
    ClassRefused {
        /// The refused class.
        class: ContactClass,
    },
    /// The chosen primitive and rider have no row in the coset table
    /// ([`table_gap`]) — a clocking rider on a planar rest, a
    /// standalone clocking with no carrying mate — a fact about the
    /// choice alone, so it is refused HERE before any geometry is
    /// read, the class door's shape one row down, in the table's own
    /// words. What is NOT this: a rider on a frame coincidence, which
    /// the table DECIDES over the mate's lever and the edit door
    /// refuses when it contradicts (`EditError::MateRefused`),
    /// surfaced by `perform` like every door refusal.
    TableRefused {
        /// What was asked for, in the table's own words.
        what: &'static str,
    },
}

impl core::fmt::Display for MateToolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotTwoPicks => write!(f, "the mate tool needs two face picks"),
            Self::PickIsNotAFace { side, refusal } => write!(
                f,
                "pick {} does not name a face, so it is no mate head: {refusal}",
                side.name()
            ),
            Self::NotAnInstancePick { side, node } => write!(
                f,
                "pick {} is on {node}, which is not a part instance or a copy of one",
                side.name()
            ),
            Self::SamePick { head } => write!(
                f,
                "both picks name the same member (head: {head}); a mate relates a pair"
            ),
            Self::Frame { side, error, held } => write!(
                f,
                "pick {}'s face frame cannot be derived: {}",
                side.name(),
                Said(error, Speaker::held(held))
            ),
            Self::ClassRefused { class } => {
                write!(
                    f,
                    "class {} is not admitted — {CLASS_DEFERRAL}",
                    class.name()
                )
            }
            Self::TableRefused { what } => {
                write!(f, "the coset table has no row for {what}")
            }
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
            Self::NotAnInstancePick { side, node } => Self::NotAnInstancePick {
                side,
                node: node.respoken(doc),
            },
            Self::SamePick { head } => Self::SamePick {
                head: head.respoken(doc),
            },
            Self::Frame { side, error, held } => Self::Frame {
                side,
                error,
                held: held.respoken(doc),
            },
            unspoken @ (Self::NotTwoPicks
            | Self::PickIsNotAFace { .. }
            | Self::ClassRefused { .. }
            | Self::TableRefused { .. }) => unspoken,
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
    /// The first pick, held.
    One(FaceSelection),
    /// Both picks, held; the class/alignment choice is offered.
    Two {
        /// The first pick (the mate's `a` side).
        a: FaceSelection,
        /// The second pick (the mate's `b` side).
        b: FaceSelection,
    },
}

impl MateToolState {
    /// The held picks, side `a` then side `b`, `None` for a side not
    /// yet picked — what the panel's line says and the viewport marks.
    pub fn picks(&self) -> [Option<&FaceSelection>; 2] {
        match self {
            Self::Idle => [None, None],
            Self::One(a) => [Some(a), None],
            Self::Two { a, b } => [Some(a), Some(b)],
        }
    }

    /// **The line the mate panel shows for its held picks** — the
    /// seated tools' line (`seats::picks_line`), with the
    /// mate's two sides as its roles and each pick said as `face_of`
    /// says it, the drop notice's phrase.
    ///
    /// The tool's state is not [`crate::seats::Seats`] (module docs:
    /// neither the state nor the survival rule is shared), but the
    /// line is the same sentence about the same thing — a role and
    /// what fills it — so it is composed by the same door.
    pub fn line(&self, doc: &Doc<ProfileProgram>) -> String {
        let [a, b] = self.picks();
        crate::seats::picks_line([(MateSide::A, a), (MateSide::B, b)].map(|(side, pick)| {
            (
                format!("pick {}", side.name()),
                pick.map(|pick| face_of(&doc.spoken(pick.node))),
            )
        }))
    }
}

/// **What this tool calls a held pick**: `face of Extrude 000000000003`
/// — the face of the node whose body the pick was taken on, as the
/// document speaks it.
///
/// The panel item ([`MateToolState::line`]) and the drop notice
/// ([`MateToolEvent`]) both say it, about the same pick on the same
/// frame, so they read it here rather than each spelling it: two
/// copies of one phrase is how a panel and its notice come to call
/// one pick two things.
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
        /// The pick's node as the document spoke it when the pick was
        /// dropped.
        node: SpokenNode,
        /// The resolution machinery's own verdict, read as the feature
        /// tree reads it ([`crate::tree::resolution_as_drawn`]); boxed
        /// for the same width reason `Standing` boxes it.
        ///
        /// Its `through` may be a mate, which is not the DAG ancestor
        /// `NodeStanding` documents
        /// (`work/wire/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`).
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
                "pick {} (a {}) no longer resolves; the tool dropped it",
                side.name(),
                face_of(node)
            ),
        }
    }
}

/// The user's class/alignment choice — what the chrome's controls
/// select between picks and commit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MateChoice {
    /// The declared contact class.
    pub class: ContactClass,
    /// The coset primitive.
    pub primitive: MatePrimitive,
    /// Which way the two faces' axes point at each other.
    pub sense: AxisSense,
    /// The clocking rider, if authored.
    pub clocking: Option<f64>,
}

/// The derived, ready-to-commit mate: the two picked members'
/// references and the alignment in part coordinates, plus the class's
/// admission verdict for the chrome to show beside the commit.
#[derive(Debug, Clone)]
pub struct MateProposal {
    /// The `a` reference: the picked name, read at the node the ray
    /// met.
    pub a: SitedFace,
    /// The `b` reference.
    pub b: SitedFace,
    /// The declared class.
    pub class: ContactClass,
    /// The derived alignment.
    pub alignment: Alignment,
    /// The kernel's admission verdict for `class` (never
    /// `NotAdmitted` — that refuses at [`MateTool::proposal`]).
    pub admission: ClassAdmission,
}

impl MateProposal {
    /// **The one committed edit**: the session op that inserts the
    /// mate node through the ordinary commit door.
    pub fn op(&self) -> SessionOp {
        SessionOp::AddMate {
            a: self.a.clone(),
            b: self.b.clone(),
            class: self.class,
            alignment: self.alignment.clone(),
        }
    }
}

/// The modal mate tool. A value: the chrome holds one while the tool
/// is active, a test constructs one and drives the same methods.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MateTool {
    state: MateToolState,
    /// Each held pick's node as the document spoke it when the pick was
    /// taken, side `a` then side `b` — the words the panel showed, which
    /// a drop notice repeats even once the document no longer holds the
    /// node.
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
    /// into tool state. The first pick fills `a`, the second `b`; a
    /// third REPLACES `b` (the choice step is still open, and
    /// re-picking the second face is how a user corrects it).
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, face: FaceSelection) {
        let said = Some(doc.spoken(face.node));
        self.state = match std::mem::take(&mut self.state) {
            MateToolState::Idle => {
                self.said = [said, None];
                MateToolState::One(face)
            }
            MateToolState::One(a) | MateToolState::Two { a, .. } => {
                self.said[1] = said;
                MateToolState::Two { a, b: face }
            }
        };
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
        let [said_a, said_b] = std::mem::take(&mut self.said);
        let mut lost = |side: MateSide, pick: &FaceSelection| -> bool {
            let verdict =
                crate::tree::resolution_as_drawn(resolve(RunCtx { doc, eval }, &pick.name), eval);
            if resolves(&verdict) {
                false
            } else {
                events.push(MateToolEvent::PickLost {
                    side,
                    node: match side {
                        MateSide::A => said_a.clone(),
                        MateSide::B => said_b.clone(),
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
            MateToolState::One(a) => {
                if lost(MateSide::A, &a) {
                    (MateToolState::Idle, [None, None])
                } else {
                    (MateToolState::One(a), [said_a, None])
                }
            }
            MateToolState::Two { a, b } => match (lost(MateSide::A, &a), lost(MateSide::B, &b)) {
                (false, false) => (MateToolState::Two { a, b }, [said_a, said_b]),
                (false, true) => (MateToolState::One(a), [said_a, None]),
                (true, false) => (MateToolState::One(b), [said_b, None]),
                (true, true) => (MateToolState::Idle, [None, None]),
            },
        };
        events
    }

    /// Derive the committed edit from the two held picks and the
    /// user's choice: each pick's member, and its head's face as the
    /// side's frame (`MateFrame::FromFace`), after the face's pose has
    /// been read once through the shipped interrogation door as a
    /// pre-check that stores nothing.
    ///
    /// **`doc` and `eval` must be the LANDED PAIR** (the session's
    /// `landed_pair()`): the members are walked on `doc` and the
    /// pre-check reads `eval`, so a document the evaluation never saw
    /// would check a face against the wrong product. Nothing in the
    /// types can enforce the pairing; this sentence is the contract,
    /// and the application's one call site satisfies it. No
    /// evaluation options and no tolerance enter: the frame is
    /// resolved by the solve, through the evaluation's own reach, at
    /// every evaluation.
    ///
    /// # Errors
    ///
    /// Every arm of [`MateToolError`] — see each arm's docs.
    pub fn proposal(
        &self,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
        choice: MateChoice,
    ) -> Result<MateProposal, MateToolError> {
        let MateToolState::Two { a, b } = &self.state else {
            return Err(MateToolError::NotTwoPicks);
        };
        // The class door FIRST: a class the vocabulary cannot execute
        // refuses before any geometry is read, with the kernel's own
        // sentence.
        let admission = class_admission(choice.class);
        match admission {
            ClassAdmission::NotAdmitted => {
                return Err(MateToolError::ClassRefused {
                    class: choice.class,
                });
            }
            // The solve door takes both; a missing at-rest record is
            // the mint door's refusal and the tree's caveat, not this
            // door's.
            ClassAdmission::Mints | ClassAdmission::NoAtRestRecord { .. } => {}
        }
        // The table door SECOND, still before any geometry: a
        // primitive-and-rider pair the table has no row for is a fact
        // about the choice alone, read from the table's one home
        // (`table_gap`), so the tool's sentence IS the door's.
        if let Some(what) = table_gap(choice.primitive, choice.clocking) {
            return Err(MateToolError::TableRefused { what });
        }
        let (ref_a, member_a, placed_a) = picked_member(doc, MateSide::A, a)?;
        let (ref_b, member_b, placed_b) = picked_member(doc, MateSide::B, b)?;
        // MEMBERS, not nodes: two copies of one pattern are two
        // members over one instance, and a mate between them is a
        // legal (loop-closing) declaration the solve places. What a
        // mate cannot relate is a member to itself.
        if member_a == member_b {
            return Err(MateToolError::SamePick {
                head: doc.spoken(a.node),
            });
        }
        // The pre-check: the face's pose as the solve will read it, by
        // the name the walk reached at the member's instance (a copy's
        // MASTER's), refused here in the door's own words where it has
        // none. That instance's product holds only its part's rows,
        // each wrapped in its one `InPart`, so a name it answers is a
        // face of the part, which the solve reads (`head_face`). The
        // pose itself is not kept: the frame is the head's face,
        // resolved by the solve.
        let frame_of = |side: MateSide,
                        member: &Member,
                        placed: &StableName|
         -> Result<MateFrame, MateToolError> {
            face_frame(eval, member.instance, placed).map_err(|error| {
                let error = crate::tree::interrogation_as_drawn(error, eval);
                let held = held_by(&error, doc);
                MateToolError::Frame { side, error, held }
            })?;
            Ok(MateFrame::FromFace)
        };
        let frame_a = frame_of(MateSide::A, &member_a, &placed_a)?;
        let frame_b = frame_of(MateSide::B, &member_b, &placed_b)?;
        Ok(MateProposal {
            a: ref_a,
            b: ref_b,
            class: choice.class,
            alignment: Alignment {
                a: frame_a,
                b: frame_b,
                primitive: choice.primitive,
                sense: choice.sense,
                clocking: choice.clocking,
            },
            admission,
        })
    }
}
