//! **Why an operation did not happen**: the session's refusal
//! vocabulary, its ranking ladder, and the sentences the chrome shows
//! beside a refusal.
//!
//! A VOCABULARY. [`Refusal`] is a value, [`Refusal::rank`] orders two
//! of them and the wording composers are pure functions over their
//! arguments; nothing here names the session. [`NodeKindWanted`] and
//! [`admits`] live here because the kind a seat wants is a
//! [`Refusal::WrongNodeKind`] payload and `admits` is its predicate,
//! and [`Step`] for the same reason — it is the
//! [`Refusal::NothingToDo`] payload and [`Refusal::nothing_to_step`]
//! is its predicate, over the history vocabulary the moves live in.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    BooleanValue, Dimension, DimensionError, Doc, DocumentId, EditError, EvalError, Node, Operand,
    ParseError, ProfileProgram, RecipeNodeId, SlotId, SpokenNode, SpokenVar, ValuePayload, VarId,
    VarName,
};
use std::path::{Path, PathBuf};

use pncad::workspace::WorkspaceError;

// The recourse this module's `NoSuchVariable` arm ends on, read from its
// one home beside the error whose door raises the other half of the
// pair. A direct edge on the owning crate rather than a new re-export
// added to `pncad`'s root — the rule `pncad`'s own crate docs state
// for a name the facade does not carry, and the same one this crate's
// `bvh` and `Rgba8` edges cite.
use editor_core::edit::UNKNOWN_VAR_RECOURSE;

use crate::combine;
use crate::display::{AdmissionFault, DisplayFault};
use crate::docio::DocIoError;
use crate::history::History;
use crate::props::{self, Notation, SlotValue};
use crate::session::SessionOp;

/// The node kind a creation op's seat requires — the payload of
/// [`Refusal::WrongNodeKind`], so the refusal names what was wanted
/// in the vocabulary's own words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKindWanted {
    /// A `Node::Profile`.
    Profile,
    /// A read of ONE body — the combining seats' kind: a node whose one
    /// output is a body ([`combine::denotes_body`] carries the
    /// admissible set), or a split's half read by its port
    /// ([`admits_read`]).
    Body,
    /// A FAMILY — what an indexed read (`BodyRead::indexed`) indexes: a
    /// node evaluated once per value of the indices it reaches (D10
    /// `[repetition]`), or one whose output is a `Bodies` variable.
    Instances,
}

/// **Whether `node` in `doc` is the wanted kind** — the one
/// classification behind every creation seat's gate, a node `doc` does
/// not hold reading as "no", because a seat naming nothing and a seat
/// naming the wrong thing both mean there is nothing of that kind
/// there to consume.
///
/// A free function rather than a `DocSession` method because the
/// question is asked in two places for two purposes: the commit door
/// asks it to REFUSE ([`super::DocSession::require_kind`]), and a tool's
/// seats ask it to ROUTE a pick ([`crate::seats::Seats::pick`]). One
/// answer for both is what keeps a seat from steering a pick the door
/// would then reject.
pub fn admits(doc: &Doc<ProfileProgram>, node: RecipeNodeId, wanted: NodeKindWanted) -> bool {
    let held = doc.node(node);
    match wanted {
        NodeKindWanted::Body => combine::denotes_body(doc, node),
        NodeKindWanted::Instances => {
            held.is_some()
                && doc
                    .node_reach(node)
                    .chain()
                    .is_some_and(|chain| !chain.is_empty())
                || doc
                    .read_of_node(node)
                    .and_then(|read| doc.var(read))
                    .is_some_and(|var| {
                        pncad::document::SlotKind::Is(pncad::document::VarKind::Bodies).admits(var)
                    })
        }
        NodeKindWanted::Profile => held.and_then(seat_kind) == Some(wanted),
    }
}

/// **Whether `read` in `doc` is the wanted kind**: [`admits`] for a
/// node named alone, and for a port the kind of the variable it names
/// (a split's half is a body) — the gate every body seat's commit asks
/// of the read it holds.
pub fn admits_read(doc: &Doc<ProfileProgram>, read: &Operand, wanted: NodeKindWanted) -> bool {
    use pncad::document::{SlotKind, VarKind};
    let var = match read {
        Operand::Node(node) => return admits(doc, *node, wanted),
        Operand::Output { node, port } => doc.output(*node, *port),
        Operand::Var(var) => Some(*var),
        Operand::Name(name) => doc.var_named(name.as_str()),
        // A pick reads one piece of a copy: a body.
        Operand::Pick { .. } => return wanted == NodeKindWanted::Body,
        Operand::Select { .. } | Operand::Pose(_) => return false,
    };
    let kind = match wanted {
        NodeKindWanted::Body => VarKind::Body,
        NodeKindWanted::Instances => VarKind::Bodies,
        NodeKindWanted::Profile => return false,
    };
    var.and_then(|var| doc.var(var))
        .is_some_and(|var| SlotKind::Is(kind).admits(var))
}

/// **Which seat kind a node is by its kind alone**, or `None` for a
/// node no profile seat takes — the one
/// classification [`admits`] reads for every kind but the two read off
/// the variable a node named alone defines ([`NodeKindWanted::Body`],
/// [`NodeKindWanted::Instances`]).
pub(crate) fn seat_kind(node: &Node<ProfileProgram>) -> Option<NodeKindWanted> {
    match node {
        Node::Profile(_) => Some(NodeKindWanted::Profile),
        Node::Split { .. }
        | Node::Mirror { .. }
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Shell { .. }
        | Node::Subtract { .. }
        | Node::Union { .. }
        | Node::Intersect { .. }
        | Node::Assemble { .. }
        | Node::InstantiatePart { .. }
        | Node::Place { .. }
        | Node::World
        | Node::Space
        | Node::Measure { .. }
        | Node::Assertion { .. } => None,
    }
}

impl NodeKindWanted {
    /// The kind's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::Profile => "a profile",
            Self::Body => "a body",
            Self::Instances => "a pattern",
        }
    }
}

/// Which way a history step goes — the payload of
/// [`Refusal::NothingToDo`], so the refusal names the direction that
/// had nothing rather than hedging over both.
///
/// **It is the direction as a VALUE, which is why a third naming of
/// undo-and-redo earns its place.** [`super::SessionOp::Undo`] and
/// [`super::SessionOp::Redo`] are two operations and
/// [`crate::history::History`]'s `undo`/`redo` are two moves; neither
/// distinction can be passed to anything. This one can, and that is
/// what lets the direction-to-sentence map below and the toolbar's
/// two-row table each be written once instead of twice — which is the
/// whole of what this program asked for here.
///
/// It lives in this module rather than beside the moves for the
/// reason [`NodeKindWanted`] does — it is a `Refusal` payload and
/// [`Refusal::nothing_to_step`] is its predicate — and the stronger
/// reason is the one that answers the obvious alternative: folding
/// `History::can_undo`/`can_redo` into one `can_step(Step)` would
/// replace two predicates that say which way they go, at eleven
/// reading sites that know their direction already, with one that
/// makes every reader carry an argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Toward the root — [`super::SessionOp::Undo`].
    Undo,
    /// Along the current branch — [`super::SessionOp::Redo`].
    Redo,
}

impl Step {
    /// The verb, for a sentence a person reads.
    fn verb(self) -> &'static str {
        match self {
            Self::Undo => "undo",
            Self::Redo => "redo",
        }
    }
}

/// Why an operation did not happen. Every arm is a value the chrome
/// renders; none is a message this layer composed about someone else's
/// failure.
#[derive(Debug)]
pub enum Refusal {
    /// The slot is driven by an expression, so a direct numeric edit
    /// is refused with the affordance G4 ratifies for a drag, which
    /// this layer extends to a typed write. The payload is what the
    /// affordance needs: which variables drive it (each navigable and
    /// editable), and what the slot evaluates to today.
    DrivenByExpression {
        /// The node holding the slot.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The variables the driving expression reads, each as the
        /// document spoke it at the refusal.
        variables: Vec<SpokenVar>,
        /// The slot's current value, when it has one.
        current: Option<SlotValue>,
        /// The working notation the affordance reads `current` in —
        /// the one in force when the edit was refused, since nobody
        /// wrote that value.
        notation: Notation,
    },
    /// The node does not exist, or does not carry that slot.
    NoSuchSlot {
        /// The node named, as the document held it at the refusal.
        node: SpokenNode,
        /// The slot named.
        slot: SlotId,
    },
    /// No document variable with that id.
    ///
    /// A LOOKUP's not-found arm, never a pre-check: the two sites that
    /// raise it need the declaration itself — its dimension to open a
    /// gesture, its value and unit to seed a range probe — and neither
    /// commits an edit, so no door below refuses on their behalf. The
    /// value door does refuse a variable the document does not hold,
    /// and says so in editor-core's words ([`EditError::UnknownVar`],
    /// reached through [`Self::Edit`]).
    ///
    /// **One mistake reaches two sentences, and that is decided rather
    /// than left.** Typing into the value field of a variable the
    /// document no longer holds goes to the edit door; dragging its
    /// row comes here. The two cannot be made one refusal without
    /// putting back the pre-check the door already refuses — so what
    /// is converged is what the
    /// user must DO: this arm renders the same recourse the door
    /// renders — [`editor_core::edit::UNKNOWN_VAR_RECOURSE`], its
    /// one home — over the same fact. What stays apart is
    /// the frame, and it has to: the door's sentence is about an edit
    /// that was refused, and a drag has no edit behind it, so a
    /// gesture that borrowed the door's frame would report a
    /// refusal of something nobody attempted.
    NoSuchVariable(VarId),
    /// A value door that reads a FREE variable's value was pointed at a
    /// defined one: the range probe, which seeds its search from the
    /// value it would move. A defined variable holds a formula and no
    /// value of its own, so there is nothing to move; its range is the
    /// ranges of the variables it reads.
    VariableIsDefined(SpokenVar),
    /// An offer was accepted ([`SessionOp::SetSlotVariable`]) of a
    /// variable that is not on offer at that slot: the offer closed —
    /// the slot was retyped, dragged, undone — or never named it. An
    /// offer is made only about a value as it was typed, so the op
    /// refuses rather than join the slot to a variable chosen against a
    /// value it no longer holds.
    ///
    /// [`SessionOp::SetSlotVariable`]: crate::session::SessionOp::SetSlotVariable
    NotOffered(SpokenVar),
    /// [`SessionOp::SetSlotVariable`] carried a name for a variable
    /// that already has one. A name is given to share an unnamed
    /// variable (VR2); a named one is shared by the name it holds, and
    /// accepting its offer renames nothing.
    ///
    /// [`SessionOp::SetSlotVariable`]: crate::session::SessionOp::SetSlotVariable
    OfferIsNamed(SpokenVar),
    /// A variable's field was given a constant expression that does
    /// not evaluate to a value — a non-finite result, or a count past
    /// its range. Constant text typed as a value is folded here, before
    /// any door, so the evaluator's refusal is this door's to forward.
    ConstantRefused {
        /// The variable whose field was typed into.
        var: SpokenVar,
        /// The evaluator's refusal, in its own words.
        source: EvalError,
    },
    /// The New door was asked for a blank name.
    EmptyName,
    /// A creation op named a node that is not the kind its seat
    /// requires — absent, or of another kind. Refusing here keeps
    /// "that is not a profile/axis" a fact stated at the door rather
    /// than a failed node discovered after the edit lands; one arm
    /// for every seat, so the sentence is spelled once (GAUTH-4/5 add
    /// more seats to the same rule).
    WrongNodeKind {
        /// The node named, as the document held it at the refusal.
        node: SpokenNode,
        /// The kind the seat requires.
        wanted: NodeKindWanted,
    },
    /// A body seat's read names no output of the document: a selection,
    /// or a variable name nothing defines. A seat reads a node or one of
    /// its outputs.
    NotABodyRead {
        /// The read, as the session op held it.
        read: Box<Operand>,
    },
    /// A duplicate could not be placed — its input's landed value is
    /// not one body with a width ([`combine::DuplicateFault`]).
    Duplicate(combine::DuplicateFault),
    /// A gesture's frame could not be made: one of its numbers is not
    /// of its dimension or does not evaluate, its direction is none, or
    /// the body it reads the frame off offers no geometry to read
    /// ([`GestureFrameFault`]).
    Frame(Box<GestureFrameFault>),
    /// `apply` refused the edit — the door's own sentence, forwarded.
    ///
    /// **Layer 3 adds a frame and never a second opinion.** Every
    /// condition `apply` refuses is refused there and rendered in
    /// `EditError`'s words; a flat arm restating one would be two
    /// spellings of a rule with one home (`crates/viewer/README.md`).
    ///
    /// Boxed, as `Io` is below: these two payloads are an order of
    /// magnitude larger than every other arm, and a refusal is
    /// returned by value from functions on the ordinary path — so the
    /// unboxed shape made every `Ok` in this module pay for the widest
    /// error nobody was raising.
    Edit(Box<EditError>),
    /// The value was not a usable dimensioned literal.
    Dimension(DimensionError),
    /// The expression text did not parse.
    Parse(Box<ParseError>),
    /// A gesture operation arrived with no gesture in flight.
    NoGesture,
    /// A gesture is in flight, so this operation is not available.
    GestureInFlight,
    /// A gesture operation named a target that is not the open
    /// gesture's — a preview or a commit for a field other than the
    /// one being dragged.
    ///
    /// **Separate from [`Refusal::GestureInFlight`] because it answers
    /// a different question.** That one says a drag is open at all —
    /// either because the operation is unavailable while one is
    /// ([`super::SessionOp::permitted_during_value_gesture`]) or
    /// because it would open a second ([`crate::g1::Slot::begin`]) —
    /// and the driving operations are neither. This one is about this
    /// operation's own payload against this session's own gesture, and
    /// folding the two into one refusal would make the table's answer
    /// unreadable from the outcome.
    ///
    /// It carries no payload and ranks with the bookkeeping refusals
    /// for one reason: it arrives in a batch behind the
    /// `GestureInFlight` that refused the drag's begin, and that is
    /// the sentence with the remedy in it.
    WrongGesture,
    /// A file operation failed.
    Io(Box<DocIoError>),
    /// Undo at the root, or redo at the tip of the current branch.
    ///
    /// **The direction is the payload because the answer is about one
    /// direction and not both.** Undo at the root refuses while a redo
    /// is waiting on the branch the cursor just left, so a sentence
    /// naming both is false of the half that is live — and the two
    /// controls that produce these ops are separate buttons, each of
    /// which shows this refusal's words before the click
    /// ([`Self::nothing_to_step`]).
    NothingToDo {
        /// The direction asked for.
        direction: Step,
    },
    /// A display-state operation refused (a display op on an id the
    /// document does not hold, hide on a non-instance, a free-move on
    /// a copy in the world's space, a gesture out of order) — the
    /// fault's own typed vocabulary, unaltered.
    Display(DisplayFault),
    /// A written-unit change refused — the panel model's own typed
    /// vocabulary, unaltered.
    SlotUnit(props::SlotUnitFault),
    /// The session has no backing file, so there is no directory for a
    /// part reference to be picked from or to resolve against — the
    /// directory rule's consequence at authoring time. Its recourse
    /// rides the sentence, composed in `Display` like every other arm
    /// here.
    NoDocumentDirectory,
    /// The workspace refused — the scan (duplicate id, unreadable
    /// sibling) or the read of the part being referenced — in the
    /// store's own words.
    ///
    /// Boxed for [`Refusal::Edit`]'s reason: its widest arms carry two
    /// paths (a duplicate id names both claimants) or a path with two
    /// pins (a mismatch names what was wanted and what was found).
    Workspace(Box<WorkspaceError>),
    /// The open document was asked to instantiate ITSELF.
    ///
    /// Refused at the door rather than left to fail later. A
    /// self-reference pins the file's content as it stands, and what
    /// happens next depends on what that file then does: a save moves
    /// the content and the pin stops holding, while a pin that still
    /// holds sends the evaluation back into the document it started
    /// in, where the descent refuses the cycle by name. Neither
    /// outcome is one anybody asked for. `refactor`'s split door
    /// refuses a self-referencing identity in the same spirit, though
    /// for its own first reason — the produced pair could not both
    /// live in one store — with the evaluation cycle recorded beside
    /// it.
    SelfInstance {
        /// The identity that is both the open document and the part
        /// asked for.
        id: DocumentId,
    },
    /// The path editor's program was loaded from one the document no
    /// longer holds (an undo, or an edit from elsewhere, landed in
    /// between): it is an edit of something that is not there any
    /// more, and is not written over what is.
    ProfileEditStale {
        /// The profile node, as the document held it at the refusal.
        node: SpokenNode,
    },
    /// A takeover would re-point a copy a pick reads onto a body that
    /// holds no pieces — a feature's result, a mirror's or a sketch's
    /// union, a combine. A pick is never re-pointed (D10
    /// `[placement.placement]`), so it would name a piece its copy no
    /// longer holds; the takeover refuses rather than strand it.
    StrandedPick {
        /// The copy the takeover would re-point.
        copy: SpokenNode,
        /// The pick reading it.
        pick: SpokenVar,
    },
}

impl Refusal {
    /// **This refusal with every node and variable it names spoken
    /// again from `doc`** — a later version of the document it was
    /// raised in, so a label or a name changed since the raise is the
    /// one it says. The rule
    /// and why it is sound are [`SpokenNode::respoken`]'s; it is why a
    /// document an `Open` or a `New` replaced is never `doc` here
    /// (`frame::batch_refusal`).
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        let again = |node: SpokenNode| node.respoken(doc);
        match self {
            Self::NoSuchSlot { node, slot } => Self::NoSuchSlot {
                node: again(node),
                slot,
            },
            Self::WrongNodeKind { node, wanted } => Self::WrongNodeKind {
                node: again(node),
                wanted,
            },
            Self::ProfileEditStale { node } => Self::ProfileEditStale { node: again(node) },
            Self::StrandedPick { copy, pick } => Self::StrandedPick {
                copy: again(copy),
                pick: pick.respoken(doc),
            },
            Self::ConstantRefused { var, source } => Self::ConstantRefused {
                var: var.respoken(doc),
                source,
            },
            Self::VariableIsDefined(var) => Self::VariableIsDefined(var.respoken(doc)),
            Self::NotOffered(var) => Self::NotOffered(var.respoken(doc)),
            Self::OfferIsNamed(var) => Self::OfferIsNamed(var.respoken(doc)),
            Self::Duplicate(fault) => Self::Duplicate(fault.respoken(doc)),
            Self::Display(fault) => Self::Display(fault.respoken(doc)),
            Self::SlotUnit(fault) => Self::SlotUnit(fault.respoken(doc)),
            Self::Edit(error) => Self::Edit(Box::new(error.respoken(doc))),
            Self::DrivenByExpression {
                node,
                slot,
                variables,
                current,
                notation,
            } => Self::DrivenByExpression {
                node,
                slot,
                variables: variables.iter().map(|var| var.respoken(doc)).collect(),
                current,
                notation,
            },
            unspoken @ (Self::NoSuchVariable(_)
            | Self::NotABodyRead { .. }
            | Self::EmptyName
            | Self::Dimension(_)
            | Self::Frame(_)
            | Self::Parse(_)
            | Self::NoGesture
            | Self::GestureInFlight
            | Self::WrongGesture
            | Self::Io(_)
            | Self::NothingToDo { .. }
            | Self::NoDocumentDirectory
            | Self::Workspace(_)
            | Self::SelfInstance { .. }) => unspoken,
        }
    }

    /// **The parse error, when the refusal is the expression door's
    /// parse refusal** — the text an author typed did not parse, so
    /// nothing reached the document and the typed source is still the
    /// author's. The one reading `frame::creation_offer` and
    /// `frame::retype_draft` both take.
    pub fn parse_error(&self) -> Option<&ParseError> {
        match self {
            Self::Parse(error) => Some(&**error),
            Self::DrivenByExpression { .. }
            | Self::NoSuchSlot { .. }
            | Self::NoSuchVariable(_)
            | Self::ConstantRefused { .. }
            | Self::VariableIsDefined(_)
            | Self::NotOffered(_)
            | Self::OfferIsNamed(_)
            | Self::EmptyName
            | Self::WrongNodeKind { .. }
            | Self::NotABodyRead { .. }
            | Self::Duplicate(_)
            | Self::Frame(_)
            | Self::Edit(_)
            | Self::Dimension(_)
            | Self::NoGesture
            | Self::GestureInFlight
            | Self::WrongGesture
            | Self::Io(_)
            | Self::NothingToDo { .. }
            | Self::Display(_)
            | Self::SlotUnit(_)
            | Self::NoDocumentDirectory
            | Self::Workspace(_)
            | Self::SelfInstance { .. }
            | Self::ProfileEditStale { .. }
            | Self::StrandedPick { .. } => None,
        }
    }

    /// How much this refusal has to say, lower being more.
    ///
    /// **A frame performs a BATCH of operations**, and a batch can hold
    /// more than one refusal: dragging an expression-driven slot queues
    /// `BeginGesture` (refused with G4's ratified affordance) and
    /// `PreviewGesture` (refused `NoGesture`, purely because the first
    /// refusal stopped the gesture from opening). A chrome that keeps
    /// the last refusal shows the second one and buries the decision
    /// the affordance exists to deliver.
    ///
    /// So the ranks are: the affordance first, because it is a ratified
    /// decision (G4) about what the user just tried; then every refusal that
    /// names a real failure; then the bookkeeping ones, which are
    /// consequences of an earlier refusal at least as often as they are
    /// news. [`Refusal::preferred`] applies it.
    pub fn rank(&self) -> u8 {
        match self {
            Self::DrivenByExpression { .. } => 0,
            Self::NoSuchSlot { .. }
            | Self::NoSuchVariable(_)
            | Self::ConstantRefused { .. }
            | Self::VariableIsDefined(_)
            | Self::NotOffered(_)
            | Self::OfferIsNamed(_)
            | Self::EmptyName
            | Self::WrongNodeKind { .. }
            | Self::NotABodyRead { .. }
            | Self::Duplicate(_)
            | Self::Frame(_)
            | Self::Edit(_)
            | Self::Dimension(_)
            | Self::Parse(_)
            | Self::SlotUnit(_)
            | Self::NoDocumentDirectory
            | Self::Workspace(_)
            | Self::SelfInstance { .. }
            | Self::ProfileEditStale { .. }
            | Self::StrandedPick { .. }
            | Self::Io(_) => 1,
            // The ONE arm whose rank is a per-payload decision: the
            // three gesture-order faults rank with their document
            // twins, and the substantive ones rank with the real
            // failures, because "this instance is placed into the
            // world" is a decision about what the user tried; "no
            // evaluation has landed" is not, and ranks with the
            // gesture-order faults. `Edit` and
            // `SlotUnit` forward whole vocabularies at one rank each
            // and that IS a default: every condition either raises is
            // a real failure, so no payload of theirs ranks
            // differently. The admission family is walked arm by arm
            // too, rather than folded into one `Admission(_)`.
            Self::Display(fault) => match fault {
                DisplayFault::NoFreeMove
                | DisplayFault::FreeMoveInFlight
                | DisplayFault::WrongFreeMove => 2,
                DisplayFault::NonRigidFrame { .. } => 1,
                DisplayFault::Admission(fault) => match fault {
                    AdmissionFault::NotEvaluated { .. } => 2,
                    AdmissionFault::NoSuchNode { .. }
                    | AdmissionFault::NotAnInstance { .. }
                    | AdmissionFault::InTheWorld { .. }
                    | AdmissionFault::NoSpace { .. }
                    | AdmissionFault::MovedSpace { .. }
                    | AdmissionFault::NoSuchSpace { .. }
                    | AdmissionFault::NotShown { .. }
                    | AdmissionFault::FusedGeometry { .. } => 1,
                },
            },
            Self::NoGesture
            | Self::GestureInFlight
            | Self::WrongGesture
            | Self::NothingToDo { .. } => 2,
        }
    }

    /// The refusal a frame should show, given the one it already has.
    ///
    /// Strictly better wins; ties keep the incumbent, so within one
    /// rank the FIRST refusal of a frame is the one displayed — it is
    /// the one that describes what the user's action ran into, and
    /// everything after it is downstream of that.
    pub fn preferred(shown: Option<Self>, next: Self) -> Option<Self> {
        match shown {
            Some(shown) if shown.rank() <= next.rank() => Some(shown),
            _ => Some(next),
        }
    }

    /// **The self-instance rule and its refusal, in one place**:
    /// `Some` exactly when `id` is the open document `open`.
    ///
    /// Both consumers of the rule call this — the op, which refuses,
    /// and the catalogue, which marks the entry it cannot offer — so
    /// the predicate has one home and the chrome's disabled reason is
    /// the same value the click would have been answered with.
    pub fn self_instance(open: DocumentId, id: DocumentId) -> Option<Self> {
        (open == id).then_some(Self::SelfInstance { id })
    }

    /// **The nothing-to-step answer, ahead of the move**: `Some`
    /// exactly when the history has no state to move to in
    /// `direction`.
    ///
    /// This is the CHROME's door. `DocSession::step` does not call it
    /// and must not: a door reports what its own attempt found, so it
    /// moves and refuses on the `None` the move itself answers with,
    /// which is the only reading that cannot go stale between the ask
    /// and the act. What this composes is the same refusal VALUE, for
    /// the toolbar's Undo and Redo buttons, which have to decide
    /// whether to offer the move BEFORE it is attempted and owe the
    /// reader the sentence the attempt would have given.
    ///
    /// Those two buttons are the only hand that pushes
    /// [`super::SessionOp::Undo`] or [`super::SessionOp::Redo`], and
    /// they are disabled exactly while this is `Some` — so the
    /// tooltip is the only place that sentence is ever read, and the
    /// status line is not a second surface for it.
    pub fn nothing_to_step(history: &History, direction: Step) -> Option<Self> {
        let available = match direction {
            Step::Undo => history.can_undo(),
            Step::Redo => history.can_redo(),
        };
        (!available).then_some(Self::NothingToDo { direction })
    }

    /// **The new-document name rule, and its one home**: the name a
    /// [`super::SessionOp::NewDocument`] is built from, or the refusal
    /// a blank one is answered with.
    ///
    /// Both consumers call this — `DocSession::new_document`, which
    /// refuses, and the New-document form's Create button, which
    /// disables and shows the words.
    ///
    /// **It hands back the NAME rather than a verdict** because the
    /// trim is part of the rule and not each caller's own step. A
    /// control that gated on the raw text would offer a click the door
    /// refuses; a caller that trimmed again on its own would be a
    /// second copy of the normalisation with nothing holding the two
    /// in step — which is the defect one home for the emptiness alone
    /// still leaves open.
    ///
    /// The name says `new_document` and not `name`: a blank
    /// *variable* name is a different question with a different
    /// answer — no door refuses one at all
    /// (`work/edit/no-door-refuses-a-blank-parameter-name`) — and a
    /// general name here would be an inviting wrong door for it.
    pub fn new_document_name(typed: &str) -> Result<&str, Self> {
        let name = typed.trim();
        if name.is_empty() {
            Err(Self::EmptyName)
        } else {
            Ok(name)
        }
    }

    /// **The affordance sentence, and its one home.**
    ///
    /// G4 ratifies the behaviour — "Dragging an expression-driven
    /// dimension refuses, with an affordance offering to edit the
    /// expression" — and decides no wording; the sentence is this
    /// layer's. It is composed once and **every surface that shows it
    /// calls this** — through this function, or through
    /// the `Display` of a [`Refusal::DrivenByExpression`] the surface
    /// is holding. The surfaces are not listed here: a list is a
    /// census of call sites that nothing re-derives, and the rule is
    /// what does the work. Two independently-built copies is how two
    /// surfaces come to say different things about one refusal.
    ///
    /// The current value is spelled as the slot's field spells it
    /// ([`props::computed_text`], in the working `notation` and
    /// carrying its symbol), so the two never show one number two ways.
    pub fn affordance(
        variables: &[SpokenVar],
        slot: SlotId,
        current: Option<SlotValue>,
        notation: Notation,
    ) -> String {
        let over = if variables.is_empty() {
            "an expression".to_owned()
        } else {
            let names: Vec<String> = variables.iter().map(SpokenVar::to_string).collect();
            format!("an expression over {}", names.join(", "))
        };
        match current {
            Some(value) => format!(
                "driven by {over} (currently {}) — edit the expression?",
                props::computed_text(slot.expr_dimension(), value.as_f64(), notation)
            ),
            None => format!("driven by {over} — edit the expression?"),
        }
    }

    /// The already-declared sentence, and its one home: the add-
    /// variable form's notice BEFORE the click, which offers the
    /// standing variable's row in place of the Create button. A click
    /// that reaches the door anyway is refused by the declare itself
    /// (`EditError::VarNameTaken`, through [`Refusal::Edit`]); the
    /// notice is an offer the form makes from the document it reads,
    /// with the standing declaration's dimension, which the door's
    /// refusal does not carry.
    ///
    /// The dimension is named through its OWN `Display`, which is the
    /// one home of the dimension-in-prose rule (`Dimension`'s impl in
    /// editor-core): a dimension is a quantity KIND, so a sentence a
    /// person reads says the common noun and never the variant
    /// identifier.
    pub fn exists_wording(name: &VarName, dimension: Dimension) -> String {
        format!(
            "variable {} already exists ({dimension}) — edit it instead?",
            name.as_str()
        )
    }

    /// The not-declared fact, and its one home. The status line
    /// renders it through [`Refusal::NoSuchVariable`], which adds the
    /// recourse; the properties pane shows it bare, as the verdict on a
    /// variable selection that no longer denotes — one composition,
    /// so the pane describing the selection and the refusal answering
    /// a drag on it say the fact in the same words.
    ///
    /// The recourse is the refusal's half alone: it instructs an
    /// attempt, and the pane is describing a selection, not answering
    /// one.
    pub fn undeclared_wording(var: &SpokenVar) -> String {
        format!("variable {var} is not in this document")
    }

    /// The create-offer sentence, and its one home — shown over the
    /// add-variable form when an expression refused on this name.
    pub fn offer_wording(name: &VarName) -> String {
        format!("create variable {}?", name.as_str())
    }

    /// **The choice this refusal offers**, when it is a save that
    /// would give the document a second file in one folder.
    pub fn save_choice(&self) -> Option<SaveChoice> {
        match self {
            Self::Workspace(error) => match &**error {
                WorkspaceError::SaveWouldDuplicateId {
                    existing,
                    requested,
                    ..
                } => Some(SaveChoice {
                    existing: existing.clone(),
                    requested: requested.clone(),
                }),
                _ => None,
            },
            _ => None,
        }
    }

    /// The accept-offer question, and its one home — shown under an
    /// instance row whose pin no longer holds. It names the part by
    /// its file, as the row does, and says the accept reaches every
    /// instance of it.
    pub fn version_question(offer: &VersionOffer) -> String {
        format!(
            "accept the updated version of {}, at every instance of it?",
            offer.part
        )
    }
}

impl core::fmt::Display for Refusal {
    /// Renders each arm through its payload's OWN `Display` wherever
    /// the payload has one — `EditError`, `DimensionError` and
    /// `PersistError` (inside [`DocIoError`]) all do, and using them is
    /// the same rule the feature tree's badges follow: the layer that
    /// raised the failure names it.
    ///
    /// One exception, stated rather than hidden: the affordance arm's
    /// wording is this layer's own sentence, so it is
    /// composed here (and here only — [`Refusal::affordance`] is its
    /// single home).
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DrivenByExpression {
                slot,
                variables,
                current,
                notation,
                ..
            } => write!(
                f,
                "{}",
                Self::affordance(variables, *slot, *current, *notation)
            ),
            Self::NoSuchSlot { node, slot } => {
                write!(f, "{node} has no {} slot", slot.label())
            }
            Self::NoSuchVariable(var) => {
                write!(
                    f,
                    "{} — {UNKNOWN_VAR_RECOURSE}",
                    Self::undeclared_wording(&SpokenVar::new(*var, None))
                )
            }
            Self::ConstantRefused { var, source } => {
                write!(f, "the value typed for {var} does not evaluate: {source}")
            }
            Self::VariableIsDefined(var) => write!(
                f,
                "{var} is defined by a formula and holds no value of its own to move — \
                 probe a variable it reads"
            ),
            Self::NotOffered(var) => write!(
                f,
                "{var} is no longer offered here — type the value again to be offered \
                 the variables equal to it"
            ),
            Self::OfferIsNamed(var) => write!(
                f,
                "{var} already has a name, and the slot shares it by that name — accept \
                 the offer without giving it another"
            ),
            Self::EmptyName => write!(f, "a new document needs a name"),
            Self::WrongNodeKind { node, wanted } => {
                write!(f, "{node} is not {} in this document", wanted.name())
            }
            Self::NotABodyRead { read } => match &**read {
                Operand::Name(name) => write!(f, "no variable is named {name} in this document"),
                _ => f.write_str(
                    "a selection names cells of a body, not the body: a body seat reads a node \
                     or one of its outputs",
                ),
            },
            // The frame is layer 3's and the sentence is the door's.
            // Nothing is doubled: `EditError`'s arms state the problem
            // and carry no category prefix of their own, so this reads
            // as one sentence rather than as two openings.
            Self::Duplicate(fault) => write!(f, "{fault}"),
            Self::Frame(fault) => write!(f, "{fault}"),
            Self::Edit(error) => write!(f, "the edit was refused: {error}"),
            Self::Dimension(error) => write!(f, "{error}"),
            Self::Parse(error) => write!(f, "the expression did not parse: {error}"),
            Self::NoGesture => write!(f, "no drag is in progress"),
            Self::GestureInFlight => write!(f, "finish the drag first"),
            Self::WrongGesture => write!(f, "that is not the drag in progress"),
            Self::Io(error) => write!(f, "{error}"),
            Self::NothingToDo { direction } => write!(f, "nothing to {}", direction.verb()),
            Self::Display(fault) => write!(f, "{fault}"),
            Self::SlotUnit(fault) => write!(f, "{fault}"),
            Self::NoDocumentDirectory => write!(
                f,
                "save the document first — references resolve against the file's directory"
            ),
            Self::Workspace(error) => write!(f, "{error}"),
            Self::SelfInstance { id } => write!(
                f,
                "document {id} is the open document — a document cannot be an instance of \
                 itself; pick another part"
            ),
            Self::ProfileEditStale { node } => write!(
                f,
                "{node} changed since the editor loaded it; the editor's program was not \
                 written — the editor now shows the profile as it is"
            ),
            Self::StrandedPick { copy, pick } => write!(
                f,
                "{pick} picks a piece of {copy}, and what would take {copy} over holds no \
                 pieces: a pick is never re-pointed, so the takeover would leave it naming a \
                 piece its copy no longer holds"
            ),
        }
    }
}

impl core::error::Error for Refusal {}

/// **The choice a save that would give the document a second file
/// puts in front of the person** (DI4), drawn from
/// `SaveWouldDuplicateId`'s own payload: save over the file that
/// already holds this document, or save at the chosen name as a new
/// document. Dismissing it is the third answer and emits nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveChoice {
    existing: PathBuf,
    requested: PathBuf,
}

impl SaveChoice {
    /// The label of the control that saves over the existing file.
    pub fn over_label(&self) -> String {
        format!("Save over {}", file_name(&self.existing))
    }

    /// The label of the control that forks at the chosen name.
    pub fn fork_label(&self) -> String {
        format!("Save {} as a new document", file_name(&self.requested))
    }

    /// Saving over the file that holds this document already.
    pub fn over(&self) -> SessionOp {
        SessionOp::Save(self.existing.clone())
    }

    /// Saving at the chosen name as a new document.
    pub fn fork(&self) -> SessionOp {
        SessionOp::SaveAsNewDocument(self.requested.clone())
    }
}

/// A path as a label names it: its file name, both files of a
/// [`SaveChoice`] being in one folder.
fn file_name(path: &Path) -> std::borrow::Cow<'_, str> {
    path.file_name()
        .map_or_else(|| path.to_string_lossy(), |name| name.to_string_lossy())
}

/// **The offer an instance whose pin no longer holds makes**: accept
/// its part's updated version ([`crate::frame::version_offer`] reads
/// it off the instance's own failure).
///
/// It names the part and nothing about versions: the failure line it is
/// drawn under already names both pins in the store's words, and the
/// version accepted is the one on disk at the click.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionOffer {
    id: DocumentId,
    part: String,
}

impl VersionOffer {
    /// **The button that accepts the offer**: the name the store's
    /// recourse gives the edit (`pncad::workspace::PIN_MISMATCH_RECOURSE`,
    /// "record the \"accept updated version\" edit"), so the sentence on
    /// the row and the control under it name one act.
    pub const LABEL: &str = "Accept updated version";

    /// The offer for the part `id`, named by its file `part`.
    pub(crate) fn new(id: DocumentId, part: String) -> Self {
        Self { id, part }
    }

    /// **Accepting the offer**: every reference to the part moved onto
    /// the store's version, one action at the session door, so one
    /// undo.
    pub fn accept(&self) -> SessionOp {
        SessionOp::AcceptPartVersion { id: self.id }
    }
}

/// **Why a gesture's frame could not be made** — the transform
/// gesture's placement, a split's cutting plane, a circular pattern's
/// axis. Each is made of numbers the gesture carries, taken at their
/// nominal values in the document, and read off a body's own geometry
/// (spec §5, the restate ladder).
#[derive(Debug, Clone)]
pub enum GestureFrameFault {
    /// A number is not of the dimension its field takes.
    Dimension {
        /// The field, as the form names it.
        field: &'static str,
        /// The dimension it takes.
        wanted: Dimension,
        /// The dimension it was written in.
        found: Dimension,
    },
    /// A number reads a name the document does not hold at its kind.
    Unresolved {
        /// The field, as the form names it.
        field: &'static str,
        /// The lowering door's refusal.
        fault: pncad::document::LowerFault,
    },
    /// A number did not evaluate in the document.
    Unevaluated {
        /// The field, as the form names it.
        field: &'static str,
        /// The evaluator's refusal.
        error: EvalError,
    },
    /// A direction names no direction (the direction door's refusal).
    Direction {
        /// The field, as the form names it.
        field: &'static str,
        /// The door's refusal.
        refusal: pncad::document::AxisRefusal,
    },
    /// The body offers no geometry to read the frame off.
    Geometry(pncad::document::PinRefusal),
    /// No frame reads off a traced face and a plane face
    /// ([`pncad::trace::traced_frame`]).
    Traced(std::sync::Arc<pncad::trace::TracedFrameFault>),
}

impl core::fmt::Display for GestureFrameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Dimension {
                field,
                wanted,
                found,
            } => write!(f, "the {field} takes a {wanted}, not a {found}"),
            Self::Unresolved { field, fault } => write!(f, "the {field} does not resolve: {fault}"),
            Self::Unevaluated { field, error } => {
                write!(f, "the {field} does not evaluate: {error}")
            }
            Self::Direction { field, refusal } => write!(f, "the {field} {refusal}"),
            Self::Geometry(refusal) => write!(f, "{refusal}"),
            Self::Traced(fault) => write!(f, "{fault}"),
        }
    }
}

impl core::error::Error for GestureFrameFault {}

/// **The one body an evaluated value IS, if it is one** — the
/// evaluator's single-body operand door (`eval::wire::read_body`)
/// asked of a value the viewer is holding.
///
/// The same two shapes that door takes: a `Body` payload, or a
/// boolean's non-empty result. Everything else — a split's two sides,
/// a pattern's or a placer's instances, a profile — is a
/// value it refuses `WrongOperand`.
///
/// **Not [`crate::combine::denotes_body`]**, which answers the
/// narrower question a body SEAT asks, off the node vocabulary alone,
/// before any value exists. A seat that has an evaluation in hand can
/// ask the door's own question instead of a predicate that tracks it.
///
/// Its reader is the duplicate door (`crate::combine::duplicate_step`),
/// which measures the body it hands back.
pub(crate) fn one_body(payload: &ValuePayload<f64>) -> Option<&pncad::document::Framed<f64>> {
    match payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            Some(body)
        }
        ValuePayload::Boolean(BooleanValue::Empty)
        | ValuePayload::Profile(_)
        | ValuePayload::Split { .. }
        | ValuePayload::Instances(_)
        | ValuePayload::Family
        | ValuePayload::Measure { .. }
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_) => None,
    }
}

#[cfg(test)]
mod version_offer {
    use super::VersionOffer;
    use pncad::workspace::PIN_MISMATCH_RECOURSE;

    /// **The accept button and the store's recourse name one act**: the
    /// label is the edit's name the recourse quotes. Red if either is
    /// re-worded without the other.
    #[test]
    fn the_accept_button_is_the_edit_the_pin_mismatch_recourse_quotes() {
        let quoted = format!("\"{}\"", VersionOffer::LABEL.to_lowercase());
        assert!(
            PIN_MISMATCH_RECOURSE.contains(&quoted),
            "{quoted} in: {PIN_MISMATCH_RECOURSE}"
        );
    }
}
