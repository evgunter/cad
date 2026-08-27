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
//! # Where the numbers come from
//!
//! A mate's alignment frames are AUTHORED data in each instance's own
//! part coordinates (A11 keeps the solve structural — no geometry
//! inspection at evaluation). This tool is the door that derives that
//! authored data FROM the picked geometry, once, at authoring time:
//! `names::interrogate::face_frame` answers each picked face's world
//! pose as of the landed evaluation, and the instance's current
//! placement (the shipped constructive solve) pulls it back into part
//! coordinates. What lands in the document is plain numbers, exactly
//! as if the author had typed them — the solve stays structural, and
//! issue #944's "nothing mints an alignment frame from a selected
//! face" is the gap this closes for the viewer.
//!
//! # What the picked frames admit
//!
//! The class choice is exposed through the kernel's own
//! [`ClassAdmission`] table ([`admitted_classes`]): `Rest` mints,
//! `Tangent` solves but carries no at-rest record, and everything the
//! vocabulary cannot name — `Fit { g₀ }` first — is the
//! [`pncad::document::CLASS_DEFERRAL`] deferral, refused typed at
//! [`MateTool::proposal`] rather than discovered as a failed node
//! after the edit lands. The table is read, never restated, so the
//! tool can never advertise what the doors will not execute.
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

use pncad::document::{
    Alignment, AxisSense, CLASS_DEFERRAL, ClassAdmission, Doc, Evaluation, Frame, MateFault,
    MateFrame, MatePrimitive, MateSide, ProfileProgram, RecipeNodeId, class_admission,
    solve_document,
};
use pncad::geom_core::Tol;
use pncad::select::{ContactClass, InterrogateError, Resolution, RunCtx, face_frame, resolve};

use crate::display::is_instance;
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
/// `Fit { g₀ }` does not appear because the kernel enum has no such
/// variant to name: its absence IS the v1 deferral, and the sentence
/// for it is [`CLASS_DEFERRAL`], which [`MateToolError::ClassRefused`]
/// carries for any class the table answers
/// [`ClassAdmission::NotAdmitted`] on.
pub fn admitted_classes() -> Vec<MateAdmission> {
    [ContactClass::Rest, ContactClass::Tangent]
        .into_iter()
        .map(|class| MateAdmission {
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
    /// A pick is not on a part instance's body, so there is no
    /// instance to mate.
    NotAnInstancePick {
        /// Which pick.
        side: MateSide,
        /// The node the pick's body belongs to.
        node: RecipeNodeId,
    },
    /// Both picks are on ONE instance. A mate relates a pair; the
    /// tool refuses here rather than authoring the edit the solve
    /// would refuse as a self-mate.
    SamePick {
        /// The instance both picks name.
        instance: RecipeNodeId,
    },
    /// A picked face's frame could not be derived — the interrogation
    /// door's own refusal (an unresolved name, an N2 tie, a NURBS
    /// face with no canonical frame), unaltered.
    Frame {
        /// Which pick.
        side: MateSide,
        /// The door's refusal.
        error: InterrogateError,
    },
    /// The picked face's carrier fixes no roll reference, so the mate
    /// frame's clocking reference cannot be derived from it.
    NoReference {
        /// Which pick.
        side: MateSide,
    },
    /// The instance's current placement could not be read (its
    /// cluster's solve refused), so the world pose cannot be pulled
    /// back into part coordinates.
    Placement {
        /// Which pick.
        side: MateSide,
        /// The solve's own fault.
        fault: Box<MateFault>,
    },
    /// The chosen class is outside the vocabulary
    /// ([`ClassAdmission::NotAdmitted`]): refused HERE, before any
    /// edit exists, with the kernel's own deferral sentence.
    ClassRefused {
        /// The refused class.
        class: ContactClass,
    },
}

impl core::fmt::Display for MateToolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotTwoPicks => write!(f, "the mate tool needs two face picks"),
            Self::NotAnInstancePick { side, node } => write!(
                f,
                "pick {} is on node {}, which is not a part instance",
                side.name(),
                node.0
            ),
            Self::SamePick { instance } => write!(
                f,
                "both picks are on instance {}; a mate relates a pair of instances",
                instance.0
            ),
            Self::Frame { side, error } => write!(
                f,
                "pick {}'s face frame cannot be derived: {error:?}",
                side.name()
            ),
            Self::NoReference { side } => write!(
                f,
                "pick {}'s face fixes no roll reference, so a mate frame cannot be \
                 derived from it",
                side.name()
            ),
            Self::Placement { side, fault } => write!(
                f,
                "pick {}'s instance has no current placement: {fault}",
                side.name()
            ),
            Self::ClassRefused { class } => {
                write!(
                    f,
                    "class {} is not admitted — {CLASS_DEFERRAL}",
                    class.name()
                )
            }
        }
    }
}

impl core::error::Error for MateToolError {}

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
        /// The resolution machinery's own verdict (boxed for the same
        /// width reason `Standing` boxes it).
        resolution: Box<Resolution>,
    },
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

/// The derived, ready-to-commit mate: the two instance-qualified
/// references and the alignment in part coordinates, plus the class's
/// admission verdict for the chrome to show beside the commit.
#[derive(Debug, Clone)]
pub struct MateProposal {
    /// The `a` reference.
    pub a: pncad::prelude::StableName,
    /// The `b` reference.
    pub b: pncad::prelude::StableName,
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
            alignment: self.alignment,
        }
    }
}

/// The modal mate tool. A value: the chrome holds one while the tool
/// is active, a test constructs one and drives the same methods.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MateTool {
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
    /// into tool state. The first pick fills `a`, the second `b`; a
    /// third REPLACES `b` (the choice step is still open, and
    /// re-picking the second face is how a user corrects it).
    pub fn pick(&mut self, face: FaceSelection) {
        self.state = match std::mem::take(&mut self.state) {
            MateToolState::Idle => MateToolState::One(face),
            MateToolState::One(a) | MateToolState::Two { a, .. } => {
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
        let mut lost = |side: MateSide, pick: &FaceSelection| -> bool {
            let verdict = resolve(RunCtx { doc, eval }, &pick.name);
            if matches!(verdict, Resolution::Resolved(_)) {
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
            MateToolState::One(a) => {
                if lost(MateSide::A, &a) {
                    MateToolState::Idle
                } else {
                    MateToolState::One(a)
                }
            }
            MateToolState::Two { a, b } => match (lost(MateSide::A, &a), lost(MateSide::B, &b)) {
                (false, false) => MateToolState::Two { a, b },
                (false, true) => MateToolState::One(a),
                (true, false) => MateToolState::One(b),
                (true, true) => MateToolState::Idle,
            },
        };
        events
    }

    /// Derive the committed edit from the two held picks and the
    /// user's choice: each pick's world frame through the shipped
    /// interrogation door, pulled back into its instance's part
    /// coordinates through the instance's current placement.
    ///
    /// # Errors
    ///
    /// Every arm of [`MateToolError`] — see each arm's docs.
    pub fn proposal(
        &self,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
        tol: Tol,
        choice: MateChoice,
    ) -> Result<MateProposal, MateToolError> {
        let MateToolState::Two { a, b } = &self.state else {
            return Err(MateToolError::NotTwoPicks);
        };
        // The class door FIRST: a class the vocabulary cannot execute
        // refuses before any geometry is read, with the kernel's own
        // sentence.
        let admission = class_admission(choice.class);
        if admission == ClassAdmission::NotAdmitted {
            return Err(MateToolError::ClassRefused {
                class: choice.class,
            });
        }
        for (side, pick) in [(MateSide::A, a), (MateSide::B, b)] {
            if !is_instance(doc, pick.node) {
                return Err(MateToolError::NotAnInstancePick {
                    side,
                    node: pick.node,
                });
            }
        }
        if a.node == b.node {
            return Err(MateToolError::SamePick { instance: a.node });
        }
        // The shipped constructive solve answers each instance's
        // CURRENT placement; for a completely-unconstrained instance
        // that is its recorded (or identity) frame verbatim.
        let poses = solve_document(doc, tol);
        let frame_of = |side: MateSide, pick: &FaceSelection| -> Result<MateFrame, MateToolError> {
            let pose = face_frame(eval, pick.node, &pick.name)
                .map_err(|error| MateToolError::Frame { side, error })?;
            let u_ref = pose.u_ref.ok_or(MateToolError::NoReference { side })?;
            let placement: Frame = poses
                .placement(doc, pick.node)
                .map_err(|fault| MateToolError::Placement { side, fault })?;
            // World → part coordinates: the placement's inverse. The
            // placement is a rigid frame (the edit door and the solve
            // both hold it to that), so the inverse is exact up to
            // ordinary rounding.
            let inverse = placement.affine::<f64>().inverse();
            let origin = inverse.transform_point(pose.origin);
            let axis = inverse.transform_vec(pose.axis);
            let reference = inverse.transform_vec(u_ref);
            Ok(MateFrame {
                origin: [origin.x, origin.y, origin.z],
                axis: [axis.x, axis.y, axis.z],
                reference: [reference.x, reference.y, reference.z],
            })
        };
        let frame_a = frame_of(MateSide::A, a)?;
        let frame_b = frame_of(MateSide::B, b)?;
        Ok(MateProposal {
            a: a.name.clone(),
            b: b.name.clone(),
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
