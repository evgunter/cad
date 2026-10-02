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
    BooleanOp, BooleanValue, Datum, Dimension, DimensionError, Doc, DocumentId, EditError,
    Evaluation, HeldNodes, Node, NodeErrorKind, ParamName, ParseError, ProfileProgram,
    RecipeNodeId, Said, SlotId, Speaker, SpokenNode, ValuePayload, held_by,
};
use pncad::prelude::{Body, StableName, SurfaceKind};
use pncad::select::{FlushFinding, InterrogateError, face_carrier_kind};
use pncad::workspace::WorkspaceError;

// The recourse this module's `NoSuchParam` arm ends on, read from its
// one home beside the error whose door raises the other half of the
// pair. A direct edge on the owning crate rather than a new re-export
// added to `pncad`'s root — the ruling `pncad`'s own crate docs state
// for a name the facade does not carry, and the same one this crate's
// `bvh` and `Rgba8` edges cite.
use editor_core::edit::UNDECLARED_PARAM_RECOURSE;

use crate::combine;
use crate::display::{AdmissionFault, DisplayFault};
use crate::docio::DocIoError;
use crate::frame::Tone;
use crate::generation::Generation;
use crate::history::History;
use crate::props::{self, Notation, SlotValue};
use crate::session::{FaceSelection, SessionOp};

/// The node kind a creation op's seat requires — the payload of
/// [`Refusal::WrongNodeKind`], so the refusal names what was wanted
/// in the vocabulary's own words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKindWanted {
    /// A `Node::Profile`.
    Profile,
    /// A `Node::Datum(Datum::Axis)` — a world-space line, which is
    /// what a circular placement rule turns about.
    Axis,
    /// A `Node::Datum(Datum::AxisInPlane)` — an axis written in a
    /// sketch frame, which is what a revolve turns.
    ///
    /// Separate from [`Self::Axis`] because the two are separate node
    /// kinds and the evaluator's operand door refuses across them. A
    /// seat that admitted both would route a pick the door then
    /// rejects, which is the drift this vocabulary exists to prevent.
    SketchAxis,
    /// A `Node::Datum(Datum::Plane)`.
    Plane,
    /// A `Node::Datum(Datum::Frame)` or a `Node::Datum(Datum::FaceFrame)`
    /// — what a profile is drawn on. Both evaluate to a frame value.
    Frame,
    /// A node whose value is ONE body — the combining seats' kind
    /// ([`combine::denotes_body`] carries the admissible set and why a
    /// split's sides and a pattern's instances are not in it).
    Body,
    /// A `Node::Split` — the value a [`pncad::document::PartSelect::
    /// SplitHalf`] reads a half out of.
    ///
    /// The node kind IS the family here, with no placer to walk
    /// through: `eval::wire`'s placeable operand door refuses a split
    /// value outright, so a transform over a split is a failed node
    /// and never a second way to hold one.
    Split,
    /// A node whose value is a pattern's INSTANCES — what a
    /// [`pncad::document::PartSelect::Instance`] indexes.
    ///
    /// **Classified off the node kind, and that is narrower than the
    /// evaluator by one shape**: `Node::Transform` is shape-preserving
    /// over its input's value, so a transform of a pattern evaluates to
    /// `Instances` and `wire_part` would index it, while this answers
    /// `no`. The viewer cannot author that shape — its body seats
    /// refuse a pattern — but a loaded document may hold one, and the
    /// direction of the disagreement is the safe one: an honest
    /// refusal rather than a node that lands and then fails. It is the
    /// same defect [`Self::Body`] has in the other direction, wanting
    /// the same repair — read the family through the placer chain — so
    /// it is tracked on the row that already asks for it,
    /// `work/forms/body-seat-reads-through-the-placer-chain`. The row
    /// `combine_ops::the_part_seats_track_the_evaluators_part_door`
    /// asserts the disagreement by name, so the day the classifier
    /// walks the chain that row says so.
    Instances,
}

/// **Whether `held` is the wanted kind** — the one classification
/// behind every creation seat's gate, `None` (an absent node) reading
/// as "no", because a seat naming nothing and a seat naming the wrong
/// thing both mean there is nothing of that kind there to consume.
///
/// A free function rather than a `DocSession` method because the
/// question is asked in two places for two purposes: the commit door
/// asks it to REFUSE ([`super::DocSession::require_kind`]), and a tool's
/// seats ask it to ROUTE a pick ([`crate::seats::Seats::pick`]). One
/// answer for both is what keeps a seat from steering a pick the door
/// would then reject.
pub fn admits(held: Option<&Node<ProfileProgram>>, wanted: NodeKindWanted) -> bool {
    match wanted {
        NodeKindWanted::Body => held.is_some_and(combine::denotes_body),
        NodeKindWanted::Profile
        | NodeKindWanted::Axis
        | NodeKindWanted::SketchAxis
        | NodeKindWanted::Plane
        | NodeKindWanted::Frame
        | NodeKindWanted::Split
        | NodeKindWanted::Instances => held.and_then(seat_kind) == Some(wanted),
    }
}

/// **Which non-body kind a node is**, or `None` for a node no
/// profile, axis, plane, frame, split or instances seat takes — the one classification
/// [`admits`] reads for every kind but [`NodeKindWanted::Body`], whose
/// rule is [`combine::denotes_body`]'s.
fn seat_kind(node: &Node<ProfileProgram>) -> Option<NodeKindWanted> {
    match node {
        Node::Profile(_) => Some(NodeKindWanted::Profile),
        Node::Datum(datum) => match datum {
            Datum::Axis { .. } => Some(NodeKindWanted::Axis),
            Datum::AxisInPlane { .. } => Some(NodeKindWanted::SketchAxis),
            Datum::Plane { .. } => Some(NodeKindWanted::Plane),
            // Both frame kinds: a profile is drawn on a frame VALUE,
            // and a derived frame evaluates to the same value an
            // authored one does.
            Datum::Frame { .. } | Datum::FaceFrame { .. } => Some(NodeKindWanted::Frame),
            // No seat asks for a point.
            Datum::Point { .. } => None,
        },
        Node::Split { .. } => Some(NodeKindWanted::Split),
        Node::Pattern { .. } => Some(NodeKindWanted::Instances),
        Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Shell { .. }
        | Node::Boolean { .. }
        | Node::Union { .. }
        | Node::Transform { .. }
        | Node::Part { .. }
        | Node::PlacedUnion { .. }
        | Node::Declare { .. }
        | Node::InstantiatePart { .. }
        | Node::Mate { .. }
        | Node::Gauge { .. }
        | Node::Measure { .. }
        | Node::Assertion { .. } => None,
    }
}

impl NodeKindWanted {
    /// The kind's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::Profile => "a profile",
            Self::Axis => "an axis datum",
            Self::SketchAxis => "an axis datum in a sketch frame",
            Self::Plane => "a plane datum",
            Self::Frame => "a frame datum",
            Self::Body => "a body",
            Self::Split => "a split",
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
    /// is refused — the ratified affordance. The payload is what the
    /// affordance needs: which parameters drive it (each navigable and
    /// editable), and what the slot evaluates to today.
    DrivenByExpression {
        /// The node holding the slot.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The document parameters the driving expression reads.
        params: Vec<ParamName>,
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
    /// No document parameter by that name.
    ///
    /// A LOOKUP's not-found arm, never a pre-check: the two sites that
    /// raise it need the declaration itself — its dimension to open a
    /// gesture, its value and unit to seed a range probe — and neither
    /// commits an edit, so no door below refuses on their behalf. The
    /// value door does refuse an undeclared name, and says so in
    /// editor-core's words ([`EditError::DocParamNotDeclared`], reached
    /// through [`Self::Edit`]).
    ///
    /// **One mistake reaches two sentences, and that is decided rather
    /// than left.** Typing an undeclared name into the value field
    /// goes to the edit door; dragging its row comes here. The two
    /// cannot be made one refusal without putting back the pre-check
    /// the door already refuses — so what is converged is what the
    /// user must DO: this arm renders the same recourse the door
    /// renders — [`editor_core::edit::UNDECLARED_PARAM_RECOURSE`], its
    /// one home — over the same fact. What stays apart is
    /// the frame, and it has to: the door's sentence is about an edit
    /// that was refused, and a drag has no edit behind it, so a
    /// gesture that borrowed the door's frame would report a
    /// refusal of something nobody attempted.
    NoSuchParam(ParamName),
    /// A parameter's value field was given text that is not a number.
    ///
    /// **A document parameter holds a number, not an expression** —
    /// `DocParam::Continuous` holds an `f64` — so there is no
    /// `SetDocParamExpression` for such text to reach and no partial
    /// reading of it that would be honest. A slot's field takes the
    /// expression door here; a parameter's says why it has none, which
    /// is itself the affordance.
    ///
    /// **Raised only for text that PARSED.** Text that did not carries
    /// [`Self::Parse`], whose sentence names the token and its offset;
    /// re-wording it at this door would be a second opinion about a
    /// refusal the parser already made.
    ParamNotANumber {
        /// The parameter whose field was typed into.
        name: ParamName,
    },
    /// The CREATE door was asked for a name that is already declared.
    ///
    /// `DocEdit::SetDocParam` is create-or-replace and stays so at the
    /// API; this refusal is the session keeping "create" and
    /// "replace" distinct ACTS — see [`super::SessionOp::CreateParam`]. The
    /// payload carries the existing declaration's dimension so the
    /// offer can name what already stands there.
    ParamExists {
        /// The name, as asked for.
        name: ParamName,
        /// The dimension the existing declaration carries.
        dimension: Dimension,
    },
    /// The New door was asked for a blank name. The document id is
    /// derived from the name (`DocumentId::derive` — the identity
    /// ruling logged in `docs/GAUTH-LOG.md`), so a nameless document
    /// would carry an identity nobody could ever re-derive.
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
    /// A duplicate could not be placed — its input's landed value is
    /// not one body with a width ([`combine::DuplicateFault`]).
    Duplicate(combine::DuplicateFault),
    /// The boolean the door evaluated refused a contact nobody
    /// declared, so it was not committed ([`RefusedBoolean`]): the
    /// kernel's refusal and the attempt it answered, which is what
    /// makes the offer to declare it (`frame::declare_offer`).
    ///
    /// Boxed for [`Refusal::Edit`]'s reason.
    Contact(Box<RefusedBoolean>),
    /// `apply` refused the edit — the door's own sentence, forwarded.
    ///
    /// **Layer 3 adds a frame and never a second opinion.** Every
    /// condition `apply` refuses is refused there and rendered in
    /// `EditError`'s words; a flat arm restating one would be two
    /// spellings of a rule with one home (`crates/viewer/README.md`).
    /// One node in both operand seats used to be such an arm and is
    /// now this one: `Node::input_fault`'s pairwise-distinct rule is a
    /// fact about ANY node's inputs, so the boolean tool, `SetMembers`
    /// and the load validator all reach it at the same door.
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
    /// a mate-constrained instance, a gesture out of order) — the
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
}

impl Refusal {
    /// **This refusal with every node it names spoken again from
    /// `doc`** — a later version of the document it was raised in, so
    /// a label changed since the raise is the one it says. The rule
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
            Self::Duplicate(fault) => Self::Duplicate(fault.respoken(doc)),
            Self::Contact(refused) => Self::Contact(Box::new(refused.respoken(doc))),
            Self::Display(fault) => Self::Display(fault.respoken(doc)),
            Self::SlotUnit(fault) => Self::SlotUnit(fault.respoken(doc)),
            Self::Edit(error) => Self::Edit(Box::new(error.respoken(doc))),
            unspoken @ (Self::DrivenByExpression { .. }
            | Self::NoSuchParam(_)
            | Self::ParamNotANumber { .. }
            | Self::ParamExists { .. }
            | Self::EmptyName
            | Self::Dimension(_)
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
            | Self::NoSuchParam(_)
            | Self::ParamNotANumber { .. }
            | Self::ParamExists { .. }
            | Self::EmptyName
            | Self::WrongNodeKind { .. }
            | Self::Duplicate(_)
            | Self::Contact(_)
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
            | Self::ProfileEditStale { .. } => None,
        }
    }

    /// How much this refusal has to say, lower being more.
    ///
    /// **A frame performs a BATCH of operations**, and a batch can hold
    /// more than one refusal: dragging an expression-driven slot queues
    /// `BeginGesture` (refused with the ratified affordance) and
    /// `PreviewGesture` (refused `NoGesture`, purely because the first
    /// refusal stopped the gesture from opening). A chrome that keeps
    /// the last refusal shows the second one and buries the decision
    /// the affordance exists to deliver.
    ///
    /// So the ranks are: the affordance first, because it is a ratified
    /// decision about what the user just tried; then every refusal that
    /// names a real failure; then the bookkeeping ones, which are
    /// consequences of an earlier refusal at least as often as they are
    /// news. [`Refusal::preferred`] applies it.
    pub fn rank(&self) -> u8 {
        match self {
            Self::DrivenByExpression { .. } => 0,
            Self::NoSuchSlot { .. }
            | Self::NoSuchParam(_)
            | Self::ParamNotANumber { .. }
            | Self::ParamExists { .. }
            | Self::EmptyName
            | Self::WrongNodeKind { .. }
            | Self::Duplicate(_)
            | Self::Contact(_)
            | Self::Edit(_)
            | Self::Dimension(_)
            | Self::Parse(_)
            | Self::SlotUnit(_)
            | Self::NoDocumentDirectory
            | Self::Workspace(_)
            | Self::SelfInstance { .. }
            | Self::ProfileEditStale { .. }
            | Self::Io(_) => 1,
            // The ONE arm whose rank is a per-payload decision: the
            // three gesture-order faults rank with their document
            // twins, and the substantive ones rank with the real
            // failures, because "this instance is mate-constrained" is
            // a decision about what the user tried. `Edit` and
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
                    AdmissionFault::NoSuchNode { .. }
                    | AdmissionFault::NotAnInstance { .. }
                    | AdmissionFault::MateConstrained { .. }
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
    /// *parameter* name is a different question with a different
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

    /// **The ratified affordance sentence, and its one home.**
    ///
    /// "Dragging an expression-driven dimension → refuse, with an
    /// affordance" is a ratified micro-decision whose WORDING is part
    /// of the decision, so it is composed once and **every surface
    /// that shows it calls this** — through this function, or through
    /// the `Display` of a [`Refusal::DrivenByExpression`] the surface
    /// is holding. The surfaces are not listed here: a list is a
    /// census of call sites that nothing re-derives, and the rule is
    /// what does the work. Two independently-built copies is how the
    /// wording drifts from the decision.
    ///
    /// The current value is spelled as the slot's field spells it
    /// ([`props::computed_text`], in the working `notation` and
    /// carrying its symbol), so the two never show one number two ways.
    pub fn affordance(
        params: &[ParamName],
        slot: SlotId,
        current: Option<SlotValue>,
        notation: Notation,
    ) -> String {
        let over = if params.is_empty() {
            "an expression".to_owned()
        } else {
            let names: Vec<&str> = params.iter().map(ParamName::as_str).collect();
            format!("an expression over {}", names.join(", "))
        };
        match current {
            Some(value) => format!(
                "driven by {over} (currently {}) — edit the expression?",
                props::computed_text(slot.dimension(), value.as_f64(), notation)
            ),
            None => format!("driven by {over} — edit the expression?"),
        }
    }

    /// The already-declared sentence, and its one home. The status
    /// line renders it through [`Refusal::ParamExists`], and the add-
    /// parameter form shows the same sentence BEFORE the click — one
    /// composition, so the pre-click notice and the refusal cannot
    /// drift apart.
    ///
    /// The dimension is named through its OWN `Display`, which is the
    /// one home of the dimension-in-prose rule (`Dimension`'s impl in
    /// editor-core): a dimension is a quantity KIND, so a sentence a
    /// person reads says the common noun and never the variant
    /// identifier.
    pub fn exists_wording(name: &ParamName, dimension: Dimension) -> String {
        format!(
            "parameter {} already exists ({dimension}) — edit it instead?",
            name.as_str()
        )
    }

    /// The create-offer sentence, and its one home — shown over the
    /// add-parameter form when an expression refused on this name.
    pub fn offer_wording(name: &ParamName) -> String {
        format!("create parameter {}?", name.as_str())
    }

    /// The declare-offer question, and its one home — shown over the
    /// pairs the offer declares, in the boolean tool.
    pub fn declare_question(offer: &DeclareOffer) -> String {
        let contacts = offer.findings.iter().all(|f| f.class.contact().is_some());
        let what = match (offer.findings.len(), contacts) {
            (1, true) => "this contact",
            (_, true) => "these contacts",
            (1, false) => "this pair",
            (_, false) => "these pairs",
        };
        format!("declare {what} and commit the boolean?")
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

    /// **One pair an offer declares, as the panel names it**: each
    /// side's operand through the chrome's one spelling of a node, and
    /// the class the declaration asserts: a contact's class, or a
    /// continuation (one surface carried on, which is not a contact).
    /// The face within each operand has no prose name
    /// (`work/doors/face-pick-cannot-name-which-face.md`), so the line
    /// says "a face of" rather than inventing one.
    pub fn declare_pair_wording(doc: &Doc<ProfileProgram>, finding: &FlushFinding) -> String {
        let (one, other) = &finding.pair;
        let what = match finding.class.contact() {
            Some(class) => format!("{} contact", class.name()),
            None => "a continuation".to_owned(),
        };
        format!(
            "a face of {} against a face of {} — {what}",
            doc.spoken(one.at),
            doc.spoken(other.at),
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
    /// wording is a RATIFIED decision of this layer's, so it is
    /// composed here (and here only — [`Refusal::affordance`] is its
    /// single home).
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DrivenByExpression {
                slot,
                params,
                current,
                notation,
                ..
            } => write!(
                f,
                "{}",
                Self::affordance(params, *slot, *current, *notation)
            ),
            Self::NoSuchSlot { node, slot } => {
                write!(f, "{node} has no {} slot", slot.label())
            }
            Self::NoSuchParam(name) => {
                write!(
                    f,
                    "no document parameter named {} — {UNDECLARED_PARAM_RECOURSE}",
                    name.as_str()
                )
            }
            Self::ParamNotANumber { name } => {
                write!(
                    f,
                    "parameter {} holds a number, not an expression — write a number, with a \
                     unit if you want one (50 mm)",
                    name.as_str()
                )
            }
            Self::ParamExists { name, dimension } => {
                write!(f, "{}", Self::exists_wording(name, *dimension))
            }
            Self::EmptyName => {
                write!(
                    f,
                    "a new document needs a name; its identity is derived from it"
                )
            }
            Self::WrongNodeKind { node, wanted } => {
                write!(f, "{node} is not {} in this document", wanted.name())
            }
            // The frame is layer 3's and the sentence is the door's.
            // Nothing is doubled: `EditError`'s arms state the problem
            // and carry no category prefix of their own, so this reads
            // as one sentence rather than as two openings.
            Self::Duplicate(fault) => write!(f, "{fault}"),
            Self::Contact(refused) => write!(f, "{refused}"),
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
        }
    }
}

impl core::error::Error for Refusal {}

/// **A boolean the session door evaluated and did not commit, because
/// it refused a contact nobody declared** — [`Refusal::Contact`]'s
/// payload, and the one source of the offer to declare that contact.
///
/// It holds the attempt — the operation, its two operands, the findings
/// it already declared, the generation it was judged at — beside the
/// kernel's refusal as the kernel raised it, so the sentence a person
/// reads is the kernel's own and the offer declares exactly the pair
/// that sentence is about.
#[derive(Debug)]
pub struct RefusedBoolean {
    op: BooleanOp,
    a: RecipeNodeId,
    b: RecipeNodeId,
    declared: Vec<FlushFinding>,
    at: Generation,
    /// Always the kernel's `NodeErrorKind::UndeclaredCoincidence`: the one
    /// constructor admits nothing else.
    refused: NodeErrorKind,
    /// The nodes `refused` names, as the document the boolean was
    /// judged in held them ([`held_by`]).
    held: HeldNodes,
}

impl RefusedBoolean {
    /// **What the boolean `op` of `a` and `b`, evaluated as `node` in
    /// `eval` (of `doc`) with `declared` already declared, refused** — `None` when
    /// the node has no failure of its own ([`crate::tree::own_error`]:
    /// a poisoned node's cause is an ancestor's, sited at that
    /// ancestor's operands), or fails for any reason but an undeclared
    /// contact the node can declare.
    ///
    /// A contact between two faces of ONE operand is not one it can:
    /// the pair boolean resolves a declared pair only across its two
    /// operands, so declaring it would commit a boolean that refuses
    /// the declaration instead. That refusal stays the node's own.
    pub(crate) fn read(
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
        node: RecipeNodeId,
        attempt: (BooleanOp, [RecipeNodeId; 2]),
        declared: Vec<FlushFinding>,
        at: Generation,
    ) -> Option<Self> {
        Self::of(
            doc,
            &crate::tree::own_error(node, eval)?.kind,
            attempt,
            declared,
            at,
        )
    }

    /// [`Self::read`]'s judgement of the node's own error `refused`,
    /// its nodes said as `doc` holds them.
    fn of(
        doc: &Doc<ProfileProgram>,
        refused: &NodeErrorKind,
        (op, [a, b]): (BooleanOp, [RecipeNodeId; 2]),
        declared: Vec<FlushFinding>,
        at: Generation,
    ) -> Option<Self> {
        let NodeErrorKind::UndeclaredCoincidence {
            finding,
            merged,
            diag,
        } = refused
        else {
            return None;
        };
        if finding.pair.0.at == finding.pair.1.at {
            return None;
        }
        Some(Self {
            op,
            a,
            b,
            declared,
            at,
            refused: NodeErrorKind::UndeclaredCoincidence {
                finding: finding.clone(),
                merged: merged.clone(),
                diag: *diag,
            },
            held: held_by(refused, doc),
        })
    }

    /// This refusal with its nodes spoken from `doc`, a later version of
    /// the document it was judged in ([`Refusal::respoken`]).
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        Self {
            held: self.held.respoken(doc),
            ..self
        }
    }

    /// The finding the kernel refused: the pair and the class a
    /// declaration of it asserts.
    pub fn finding(&self) -> &FlushFinding {
        let NodeErrorKind::UndeclaredCoincidence { finding, .. } = &self.refused else {
            unreachable!("`RefusedBoolean::read` admits only an undeclared contact")
        };
        finding
    }

    /// **Whether the kernel refused a pair the attempt already
    /// declared** — its own declaration, which no further declaring can
    /// answer.
    fn re_raised(&self) -> bool {
        self.declared.contains(self.finding())
    }

    /// **The offer this refusal makes**: the same boolean again,
    /// declaring what the attempt declared and the finding it refused.
    /// `None` when the finding is one the attempt already declared.
    pub fn offer(&self) -> Option<DeclareOffer> {
        (!self.re_raised()).then(|| DeclareOffer {
            at: self.at,
            op: self.op,
            a: self.a,
            b: self.b,
            findings: self
                .declared
                .iter()
                .chain([self.finding()])
                .cloned()
                .collect(),
        })
    }
}

impl core::fmt::Display for RefusedBoolean {
    /// The kernel's sentence, whole — it names what refused and why,
    /// and adds no opening of its own. A pair the attempt already
    /// declared says so after it: that is the kernel refusing its own
    /// declaration, which no recourse in the sentence can answer.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Said(&self.refused, Speaker::held(&self.held)))?;
        if self.re_raised() {
            write!(
                f,
                " — but that pair is already declared on this boolean. {}",
                pncad::geom_core::KERNEL_DEFECT_ENDING
            )?;
        }
        Ok(())
    }
}

/// **The offer a [`RefusedBoolean`] makes** — a value, so the panel
/// shows every pair accepting it declares before anything is
/// committed.
///
/// The class of each pair is the finding's, confirmed rather than
/// chosen: the author accepts the contact the kernel detected, and a
/// class it did not detect is not on offer.
#[derive(Clone, Debug, PartialEq)]
pub struct DeclareOffer {
    at: Generation,
    op: BooleanOp,
    a: RecipeNodeId,
    b: RecipeNodeId,
    findings: Vec<FlushFinding>,
}

impl DeclareOffer {
    /// The button that accepts the offer.
    pub const ACCEPT_LABEL: &str = "Declare";

    /// The button that drops the offer and declares nothing.
    pub const DECLINE_LABEL: &str = "Decline";

    /// Every finding accepting the offer declares, in the order the
    /// refusals reported them.
    pub fn findings(&self) -> &[FlushFinding] {
        &self.findings
    }

    /// **Whether the offer still stands**: the session is at the
    /// generation the boolean was refused at — no edit, undo or open
    /// since — and the tool holds the same operation over the same two
    /// picks. An offer failing either is about a document or a pair of
    /// operands nobody is looking at.
    pub fn is_for(
        &self,
        now: Generation,
        op: BooleanOp,
        a: Option<RecipeNodeId>,
        b: Option<RecipeNodeId>,
    ) -> bool {
        self.at == now && self.op == op && a == Some(self.a) && b == Some(self.b)
    }

    /// **Accepting the offer**: the boolean again, declaring every
    /// finding — one action at the session door, so one undo.
    pub fn accept(&self) -> SessionOp {
        SessionOp::AddBoolean {
            op: self.op,
            a: self.a,
            b: self.b,
            declare: self.findings.clone(),
        }
    }
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

/// **"Nothing is picked yet", for a frame on a face — one string, two
/// readers.**
///
/// [`FaceFrameFault::NoFace`]'s sentence and the unmet-seat sentence
/// [`crate::forms::DatumKindChoice`] gives that kind are the SAME
/// sentence: the gate's "no face" and the form's "still waiting for a
/// pick" are one state said from two sides. Two literals would be two
/// spellings to keep in step, with the form suppressing one of them
/// and nothing able to notice they had drifted, so there is one
/// literal and nothing to keep in step.
pub const NO_FACE_PICKED: &str = "pick a face in the viewport to read the frame off";

/// **Why a face frame may not be authored on the face the viewport
/// has picked** — the add-datum form's `frame on face` affordance,
/// as a value.
///
/// Every arm is a fact about the pick and the landed evaluation, and
/// none is a sentence a widget composed: the form renders these and
/// decides nothing, so a headless row can drive the same question the
/// button asks ([`face_frame_seat`]).
///
/// **An AFFORDANCE, never the safety.** `Datum::FaceFrame` refuses a
/// non-planar carrier and a name that stops resolving at its own
/// evaluation (DM1b), and an `at` whose value is not one body at the
/// evaluator's single-body operand door. This says the same things one
/// step earlier, so the author learns before the node lands rather
/// than from a badge on it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaceFrameFault {
    /// Nothing is picked, or what is picked is not a face. A frame on
    /// a face has no seat to fall back on: the face IS the pick.
    NoFace,
    /// Nothing has landed yet, so there is no evaluation to read the
    /// face against. Distinct from a face that fails to resolve — one
    /// is the document not having been answered yet, the other is an
    /// answer that does not hold this name.
    NotLanded,
    /// The node the face was picked on EVALUATED to more than one
    /// body, so it is not a node a face frame may be read out of: a
    /// split's sides, a pattern's instances and a placer's map of
    /// either are several bodies, and the frame node's `at` goes
    /// through the evaluator's single-body operand door. The recipe's
    /// way of naming one of several is `Node::Part`.
    ///
    /// **A question about the VALUE, which is the door's own
    /// question.** A node-kind predicate answers a different one:
    /// `Node::Transform` is shape-preserving over its input, so a
    /// transform of a pattern is body-denoting by kind and several
    /// bodies by value.
    NotOneBody {
        /// The node whose body the ray met, as the landed document
        /// held it.
        at: SpokenNode,
    },
    /// The name does not resolve to one face in this evaluation — a
    /// stale pick, a tie, a failed node, or a node this document no
    /// longer holds at all. The interrogation door's own refusal,
    /// CARRIED rather than flattened to a string here, for
    /// [`crate::drafts::CommitFault`]'s reason.
    ///
    /// **A pick whose node an undo took away arrives here**, as
    /// [`InterrogateError::Standing`] carrying
    /// [`pncad::document::NodeStanding::NotInDocument`] — the standing of
    /// a node id the evaluated document does not have. It is not
    /// [`Self::NotOneBody`]: "several bodies" is a claim about a value
    /// that exists, and telling an author to project the one they mean
    /// would be advice about a feature that is gone.
    Unresolved {
        /// The interrogation door's refusal, read as the feature tree
        /// reads it ([`crate::tree::interrogation_as_drawn`]).
        ///
        /// Its `through` may be a mate, which is not the DAG ancestor
        /// `NodeStanding` documents
        /// (`work/wire/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`).
        error: InterrogateError,
        /// The nodes `error` names, as the landed document held them
        /// ([`held_by`]).
        held: HeldNodes,
    },
    /// The face's carrier is not a plane, and a sketch frame wants
    /// one. Names the kind it actually is, because "not a plane" alone
    /// leaves the author guessing which of their faces is curved.
    NotPlanar {
        /// The carrier kind the tag read answered.
        carrier: SurfaceKind,
    },
    /// The face resolves, but the picture does not draw it: a later
    /// feature consumed its body, or its instance is hidden. The
    /// viewport marks no held face it cannot see
    /// ([`crate::marks::drawn_patch`]), and a frame is not authored on
    /// a face nothing on screen is marking.
    NotDrawn,
}

impl FaceFrameFault {
    /// This fault with its nodes spoken from `doc`, a later version of
    /// the document it was raised in ([`Refusal::respoken`]).
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        match self {
            Self::NotOneBody { at } => Self::NotOneBody {
                at: at.respoken(doc),
            },
            Self::Unresolved { error, held } => Self::Unresolved {
                error,
                held: held.respoken(doc),
            },
            unspoken @ (Self::NoFace
            | Self::NotLanded
            | Self::NotPlanar { .. }
            | Self::NotDrawn) => unspoken,
        }
    }

    /// **How loud the datum form draws this fault** — the salience a
    /// surface reads off the value, as
    /// [`crate::session::Standing::tone`] reads it off a selection.
    ///
    /// A fault about the PICK is [`Tone::Actionable`]: a face on
    /// several bodies, a curved face, and a name that does not resolve
    /// each refuse what the reader chose, and the form stays shut until
    /// they choose again. [`Self::Unresolved`] is that on its own
    /// merits: the form's pick is LATCHED (it outlives the selection,
    /// so the reader can go on clicking elsewhere), and a latched face
    /// that no longer resolves withholds the button until the reader picks
    /// a face again, whatever is selected now.
    ///
    /// A seat not yet answerable is [`Tone::Advisory`]: no face picked
    /// is the form asking for one, and no evaluation yet is "we cannot
    /// tell", which the landing evaluation answers on its own.
    #[must_use]
    pub fn tone(&self) -> Tone {
        match self {
            Self::NoFace | Self::NotLanded => Tone::Advisory,
            Self::NotOneBody { .. }
            | Self::Unresolved { .. }
            | Self::NotPlanar { .. }
            | Self::NotDrawn => Tone::Actionable,
        }
    }
}

impl core::fmt::Display for FaceFrameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoFace => f.write_str(NO_FACE_PICKED),
            Self::NotLanded => {
                f.write_str("the document has not evaluated yet, so the picked face cannot be read")
            }
            Self::NotOneBody { at } => write!(
                f,
                "{at}'s value is several bodies, so a face on it names no single body to read \
                 a frame out of — project the one you mean first"
            ),
            Self::Unresolved { error, held } => write!(
                f,
                "that face does not resolve: {}",
                Said(error, Speaker::held(held))
            ),
            Self::NotPlanar { carrier } => write!(
                f,
                "a sketch frame is read off a PLANAR face, and that one's carrier is a {} — \
                 the kernel's own word for it",
                carrier.name()
            ),
            Self::NotDrawn => f.write_str(
                "that face is not in the picture — a later feature consumed its body, or it is \
                 hidden — so pick a face where it is drawn",
            ),
        }
    }
}

impl core::error::Error for FaceFrameFault {}

/// **May a face frame be authored on this pick, and if not, why not?**
/// — the add-datum form's `frame on face` gate, asked of values.
///
/// A free function beside [`admits`] for [`admits`]' own reason: the
/// question is about a pick and an evaluation, not about a session, so
/// a headless row asks it exactly as the widget does. The widget
/// RENDERS the answer and decides nothing.
///
/// `Ok` carries the two picks the seat needs — the node whose body the
/// ray met and the frozen face name — in the order
/// [`crate::session::DatumSpec::FaceFrame`] takes them.
///
/// # Errors
///
/// Every [`FaceFrameFault`]: no face picked, nothing landed, an `at`
/// whose VALUE is several bodies, a name that does not resolve (which
/// includes an `at` this document no longer holds), and a carrier that
/// is not a plane.
pub fn face_frame_seat(
    landed: Option<(&Doc<ProfileProgram>, &Evaluation<f64>)>,
    picked: Option<&FaceSelection>,
) -> Result<(RecipeNodeId, StableName), FaceFrameFault> {
    let face = picked.ok_or(FaceFrameFault::NoFace)?;
    // The PAIR is what a session hands out; every question below is
    // of the evaluation, including whether the document still holds
    // the node — an evaluation has no result for a node its document
    // does not have, and the interrogation door says so in its own
    // words.
    let (doc, ev) = landed.ok_or(FaceFrameFault::NotLanded)?;
    // `at` is the node whose BODY the ray met, never the feature that
    // minted the face: the name is read out of that body's own table,
    // and a flat shrunk by a later fillet is a smaller face there than
    // the feature that swept it holds.
    let at = face.node;
    // The evaluator's operand door is over the VALUE, so this is too.
    // A node KIND predicate answers a different question: `Transform`
    // is shape-preserving over its input, so a transform of a pattern
    // is body-denoting by kind and `Instances` by value, and a seat
    // gated on the kind would mint a node that refuses on arrival.
    //
    // A node with no value AT ALL — the one an undo took away, most of
    // all — is left to the interrogation below, which names why it has
    // none. "Several bodies, project the one you mean" would be advice
    // about a feature that is gone.
    if ev
        .value(at)
        .is_some_and(|value| one_body(&value.payload).is_none())
    {
        return Err(FaceFrameFault::NotOneBody { at: doc.spoken(at) });
    }
    // DM1b as a TAG READ, consulting no number: the same comparison
    // the node itself makes at evaluation.
    match face_carrier_kind(ev, at, &face.name) {
        Ok(SurfaceKind::Plane) => Ok((at, face.name.clone())),
        Ok(carrier) => Err(FaceFrameFault::NotPlanar { carrier }),
        Err(error) => {
            let error = crate::tree::interrogation_as_drawn(error, ev);
            let held = held_by(&error, doc);
            Err(FaceFrameFault::Unresolved { error, held })
        }
    }
}

/// **[`face_frame_seat`], asked of the picture on screen too**: the
/// seat the add-datum form's button commits, refusing
/// [`FaceFrameFault::NotDrawn`] for a face that resolves but that the
/// picture does not draw.
///
/// "Drawn" is [`crate::marks::drawn_patch`]'s answer — the one the
/// held mark lights — so the button and the mark cannot disagree about
/// a face while there is a picture to ask. With no index for the
/// picture on screen (`on_screen` is `None`) there is nothing to ask,
/// and the evaluation's answer stands: the button is let through
/// rather than withheld while the picture has no index — the same
/// window in which the selection's own marks light nothing.
///
/// # Errors
///
/// Every [`face_frame_seat`] refusal first, so a face that is gone is
/// said as gone; then [`FaceFrameFault::NotDrawn`].
pub fn face_frame_seat_drawn(
    landed: Option<(&Doc<ProfileProgram>, &Evaluation<f64>)>,
    picked: Option<&FaceSelection>,
    on_screen: Option<(&crate::pickindex::PickIndex, &crate::display::DisplayView)>,
) -> Result<(RecipeNodeId, StableName), FaceFrameFault> {
    let seat = face_frame_seat(landed, picked)?;
    match (picked, on_screen) {
        (Some(face), Some((index, display)))
            if crate::marks::drawn_patch(index, display, face).is_none() =>
        {
            Err(FaceFrameFault::NotDrawn)
        }
        _ => Ok(seat),
    }
}

/// **The one body an evaluated value IS, if it is one** — the
/// evaluator's single-body operand door (`eval::wire::body_operand`)
/// asked of a value the viewer is holding.
///
/// The same two shapes that door takes: a `Body` payload, or a
/// boolean's non-empty result. Everything else — a split's two sides,
/// a pattern's or a placer's instances, a datum, a profile — is a
/// value it refuses `WrongOperand`.
///
/// **Not [`crate::combine::denotes_body`]**, which answers the
/// narrower question a body SEAT asks, off the node vocabulary alone,
/// before any value exists. A seat that has an evaluation in hand can
/// ask the door's own question instead of a predicate that tracks it.
///
/// Two readers: the face-frame seat, which needs only the answer, and
/// the duplicate door (`crate::combine::duplicate_step`), which
/// measures the body it hands back.
pub(crate) fn one_body(payload: &ValuePayload<f64>) -> Option<&Body<f64>> {
    match payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            Some(body)
        }
        ValuePayload::Boolean(BooleanValue::Empty)
        | ValuePayload::Datum(_)
        | ValuePayload::Profile(_)
        | ValuePayload::Split { .. }
        | ValuePayload::Instances(_)
        | ValuePayload::Mate(_)
        | ValuePayload::Gauge
        | ValuePayload::Measure { .. }
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_) => None,
    }
}

/// **What a boolean's contact refusal offers, judged on the kernel's own
/// refusal** — the two cases no scene through the session door reaches:
/// a pair the attempt already declares, and a pair between two faces of
/// one operand. Each starts from the boss scene's plain union, the real
/// refusal, and changes the one thing its case is about.
#[cfg(test)]
mod refused_boolean {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::document::{BooleanOp, Doc, Node, NodeErrorKind, ProfileProgram, RecipeNodeId};
    use pncad::geom_core::{KERNEL_DEFECT_ENDING, Tol};

    use super::RefusedBoolean;
    use crate::generation::Generation;
    use crate::test_support::{boss_on_block, inserted_and_evaluated};

    /// The boss scene's union refusal as the kernel raised it, handed
    /// to `with` beside the document the union was evaluated in — its
    /// evaluation lives only as long as this call.
    fn with_refusal<R>(
        with: impl FnOnce(&Doc<ProfileProgram>, &NodeErrorKind, [RecipeNodeId; 2]) -> R,
    ) -> R {
        let tol = Tol::witness();
        let (doc, block, boss) = boss_on_block("refused-boolean", tol);
        let (judged, eval, union) = inserted_and_evaluated(
            &doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a: block,
                b: boss,
                declare: Vec::new(),
            },
            tol,
        );
        let kind = &crate::tree::own_error(union, &eval)
            .expect("the plain union fails on its own")
            .kind;
        with(&judged, kind, [block, boss])
    }

    /// **A pair the attempt already declares is not offered again**: the
    /// kernel refusing its own declaration is a defect, and the refusal
    /// says so. Red if the offer repeats the pair, or the sentence does
    /// not name the defect.
    #[test]
    fn a_pair_already_declared_is_refused_as_a_defect_not_offered() {
        with_refusal(|doc, kind, operands| {
            let first = RefusedBoolean::of(
                doc,
                kind,
                (BooleanOp::Union, operands),
                Vec::new(),
                Generation::FIRST,
            )
            .expect("the premise: the plain union's refusal is offerable");
            assert!(first.offer().is_some(), "the premise: it offers");
            let again = RefusedBoolean::of(
                doc,
                kind,
                (BooleanOp::Union, operands),
                vec![first.finding().clone()],
                Generation::FIRST,
            )
            .expect("still a contact refusal");
            assert_eq!(again.offer(), None, "no offer of a pair already declared");
            assert!(
                again.to_string().ends_with(KERNEL_DEFECT_ENDING),
                "and it says whose defect it is: {again}"
            );
        });
    }

    /// **A contact between two faces of one operand is not offered**: the
    /// pair boolean resolves a declared pair only across its operands, so
    /// the refusal stays the node's own. Red if it is offered.
    #[test]
    fn a_same_operand_pair_is_not_offered() {
        with_refusal(|doc, kind, operands| {
            let NodeErrorKind::UndeclaredCoincidence {
                finding,
                merged,
                diag,
            } = kind
            else {
                panic!("the premise: an undeclared contact, got {kind:?}");
            };
            let mut same = (**finding).clone();
            same.pair.1.at = same.pair.0.at;
            let one_operand = NodeErrorKind::UndeclaredCoincidence {
                finding: Box::new(same),
                merged: merged.clone(),
                diag: *diag,
            };
            assert!(
                RefusedBoolean::of(
                    doc,
                    &one_operand,
                    (BooleanOp::Union, operands),
                    Vec::new(),
                    Generation::FIRST
                )
                .is_none(),
                "a one-operand pair is left to the node's own row"
            );
        });
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
