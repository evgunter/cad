//! **Why an operation did not happen**: the session's refusal
//! vocabulary, its ranking ladder, and the sentences the chrome shows
//! beside a refusal.
//!
//! A VOCABULARY. [`Refusal`] is a value, [`Refusal::rank`] orders two
//! of them and the wording composers are pure functions over their
//! arguments; nothing here names the session. [`NodeKindWanted`] and
//! [`admits`] live here because the kind a seat wants is a
//! [`Refusal::WrongNodeKind`] payload and `admits` is its predicate.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    BooleanValue, Datum, Dimension, DimensionError, Doc, DocumentId, EditError, Evaluation, Node,
    ParamName, ParseError, ProfileProgram, RecipeNodeId, SlotId, ValuePayload,
};
use pncad::prelude::{StableName, SurfaceKind};
use pncad::select::{InterrogateError, face_carrier_kind};
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
use crate::props::{self, SlotValue};
use crate::session::FaceSelection;
use crate::sketch::Restructure;

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
    /// A `Node::Datum(Datum::Frame)` — what a profile is drawn on.
    Frame,
    /// A node whose value is ONE body — the combining seats' kind
    /// ([`combine::denotes_body`] carries the admissible set and why a
    /// split's sides and a pattern's instances are not in it).
    Body,
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
        NodeKindWanted::Profile => matches!(held, Some(Node::Profile(_))),
        NodeKindWanted::Axis => matches!(held, Some(Node::Datum(Datum::Axis { .. }))),
        NodeKindWanted::SketchAxis => {
            matches!(held, Some(Node::Datum(Datum::AxisInPlane { .. })))
        }
        NodeKindWanted::Plane => matches!(held, Some(Node::Datum(Datum::Plane { .. }))),
        // Both frame kinds: a profile is drawn on a frame VALUE, and a
        // derived frame evaluates to the same value an authored one
        // does.
        NodeKindWanted::Frame => matches!(
            held,
            Some(Node::Datum(Datum::Frame { .. } | Datum::FaceFrame { .. }))
        ),
        NodeKindWanted::Body => held.is_some_and(combine::denotes_body),
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
    },
    /// The node does not exist, or does not carry that slot.
    NoSuchSlot {
        /// The node named.
        node: RecipeNodeId,
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
        /// The node named.
        node: RecipeNodeId,
        /// The kind the seat requires.
        wanted: NodeKindWanted,
    },
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
    NothingToDo,
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
    /// The path editor's program does not have the committed
    /// profile's shape ([`crate::sketch::program_edits`]'s refusal):
    /// the document's edit vocabulary writes a program's numbers and
    /// has no door that changes its verbs, arc forms, targets or loop
    /// count. The editor locks those controls on a committed node;
    /// this is the door behind them.
    ProfileRestructure {
        /// The profile node.
        node: RecipeNodeId,
        /// Where the shapes differ.
        why: Restructure,
    },
    /// The editor's numbers are a valid profile TOGETHER — the whole
    /// program was checked before any slot was written — and no order
    /// of the one-slot writes that reach them keeps every intermediate
    /// program valid: each write re-validates the whole program, and
    /// every order was searched (`session::accepted_order`, exact up
    /// to [`super::ORDER_SEARCH_CAP`] writes). The refusal carried is
    /// the last intermediate state the search met; what the variant
    /// names is the cost of the whole-program edit the vocabulary
    /// lacks.
    ProfileEditOrder {
        /// The profile node.
        node: RecipeNodeId,
        /// The edit door's refusal of the intermediate state.
        error: Box<EditError>,
    },
    /// [`Self::ProfileEditOrder`]'s question left unanswered: the
    /// edit moves more arguments than the order search covers
    /// ([`super::ORDER_SEARCH_CAP`]), and writing them in slot order
    /// passes through a state the door refuses. Another order may
    /// land; the search for one was not run.
    ProfileEditOrderCapped {
        /// The profile node.
        node: RecipeNodeId,
        /// How many arguments the edit moves.
        writes: usize,
        /// The most the order search covers.
        cap: usize,
    },
    /// The path editor's numbers were loaded from a program the
    /// document no longer holds (an undo, or an edit from elsewhere,
    /// landed in between): they are an edit of something that is not
    /// there any more, and are not written over what is.
    ProfileEditStale {
        /// The profile node.
        node: RecipeNodeId,
    },
}

impl Refusal {
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
            | Self::ParamExists { .. }
            | Self::EmptyName
            | Self::WrongNodeKind { .. }
            | Self::Edit(_)
            | Self::Dimension(_)
            | Self::Parse(_)
            | Self::SlotUnit(_)
            | Self::NoDocumentDirectory
            | Self::Workspace(_)
            | Self::SelfInstance { .. }
            | Self::ProfileRestructure { .. }
            | Self::ProfileEditOrder { .. }
            | Self::ProfileEditOrderCapped { .. }
            | Self::ProfileEditStale { .. }
            | Self::Io(_) => 1,
            // The ONE arm whose rank is a per-payload decision, so it
            // is matched exhaustively rather than defaulted: the
            // three gesture-order faults rank with their document
            // twins,
            // and the substantive ones rank with the real failures,
            // because "this instance is mate-constrained" is a
            // decision about what the user tried. A fifth
            // `DisplayFault` reds here until its rank is chosen —
            // which is the obligation every other arm on this table
            // gets from `Refusal`'s own variants. `Edit` and
            // `SlotUnit` forward whole vocabularies at one rank each
            // and that IS a default: every condition either raises is
            // a real failure, so no payload of theirs ranks
            // differently.
            //
            // The admission family is walked arm by arm for the same
            // reason and not folded into one `Admission(_)`: that
            // spelling would be the default this arm exists to
            // refuse, one level further down, and a fifth admission
            // fault would take rank 1 unchosen.
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
            Self::NoGesture | Self::GestureInFlight | Self::WrongGesture | Self::NothingToDo => 2,
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
    pub fn affordance(params: &[ParamName], current: Option<SlotValue>) -> String {
        let over = if params.is_empty() {
            "an expression".to_owned()
        } else {
            let names: Vec<&str> = params.iter().map(|p| p.0.as_str()).collect();
            format!("an expression over {}", names.join(", "))
        };
        match current {
            Some(value) => format!(
                "driven by {over} (currently {}) — edit the expression?",
                value.as_f64()
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
            name.0
        )
    }

    /// The create-offer sentence, and its one home — shown over the
    /// add-parameter form when an expression refused on this name.
    pub fn offer_wording(name: &ParamName) -> String {
        format!("create parameter {}?", name.0)
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
                params, current, ..
            } => write!(f, "{}", Self::affordance(params, *current)),
            Self::NoSuchSlot { node, slot } => {
                write!(f, "node {} has no {} slot", node.0, slot.label())
            }
            Self::NoSuchParam(name) => {
                write!(
                    f,
                    "no document parameter named {} — {UNDECLARED_PARAM_RECOURSE}",
                    name.0
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
                write!(
                    f,
                    "node {} is not {} in this document",
                    node.0,
                    wanted.name()
                )
            }
            // The frame is layer 3's and the sentence is the door's.
            // Nothing is doubled: `EditError`'s arms state the problem
            // and carry no category prefix of their own, so this reads
            // as one sentence rather than as two openings.
            Self::Edit(error) => write!(f, "the edit was refused: {error}"),
            Self::Dimension(error) => write!(f, "{error}"),
            Self::Parse(error) => write!(f, "the expression did not parse: {error}"),
            Self::NoGesture => write!(f, "no drag is in progress"),
            Self::GestureInFlight => write!(f, "finish the drag first"),
            Self::WrongGesture => write!(f, "that is not the drag in progress"),
            Self::Io(error) => write!(f, "{error}"),
            Self::NothingToDo => write!(f, "nothing to undo or redo"),
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
            Self::ProfileRestructure { node, why } => {
                write!(f, "feature {} was not edited: {why}", node.0)
            }
            Self::ProfileEditOrder { node, error } => write!(
                f,
                "feature {}'s new numbers make a valid profile together, but every order of \
                 one-argument writes passes through a state the door refuses ({error}); the \
                 document has no edit that writes a whole program at once",
                node.0
            ),
            Self::ProfileEditOrderCapped { node, writes, cap } => write!(
                f,
                "feature {}'s new numbers make a valid profile together, but writing their {writes} \
                 arguments one at a time in order passes through a state the door refuses, and \
                 the search for another order was not run: it is capped at {cap} arguments — \
                 apply the edit in smaller steps",
                node.0
            ),
            Self::ProfileEditStale { node } => write!(
                f,
                "feature {}'s profile changed since the editor loaded it; its numbers were not \
                 written — the editor now shows the profile as it is",
                node.0
            ),
        }
    }
}

impl core::error::Error for Refusal {}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        /// The node whose body the ray met.
        at: RecipeNodeId,
    },
    /// The name does not resolve to one face in this evaluation — a
    /// stale pick, a tie, a failed node, or a node this document no
    /// longer holds at all. The interrogation door's own refusal,
    /// CARRIED rather than flattened to a string here, for
    /// [`crate::drafts::CommitFault`]'s reason.
    ///
    /// **A pick whose node an undo took away arrives here**, as
    /// [`InterrogateError::NodeNotEvaluated`] — the door's own word
    /// for a node id this evaluation has no result for. It is not
    /// [`Self::NotOneBody`]: "several bodies" is a claim about a value
    /// that exists, and telling an author to project the one they mean
    /// would be advice about a feature that is gone.
    Unresolved {
        /// The interrogation door's refusal.
        error: InterrogateError,
    },
    /// The face's carrier is not a plane, and a sketch frame wants
    /// one. Names the kind it actually is, because "not a plane" alone
    /// leaves the author guessing which of their faces is curved.
    NotPlanar {
        /// The carrier kind the tag read answered.
        carrier: SurfaceKind,
    },
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
                "feature {}'s value is several bodies, so a face on it names no single body to \
                 read a frame out of — project the one you mean first",
                at.0
            ),
            Self::Unresolved { error } => write!(f, "that face does not resolve: {error}"),
            Self::NotPlanar { carrier } => write!(
                f,
                "a sketch frame is read off a PLANAR face, and that one's carrier is a {} — \
                 the kernel's own word for it",
                carrier.name()
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
    let (_doc, ev) = landed.ok_or(FaceFrameFault::NotLanded)?;
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
        .is_some_and(|value| !is_one_body(&value.payload))
    {
        return Err(FaceFrameFault::NotOneBody { at });
    }
    // DM1b as a TAG READ, consulting no number: the same comparison
    // the node itself makes at evaluation.
    match face_carrier_kind(ev, at, &face.name) {
        Ok(SurfaceKind::Plane) => Ok((at, face.name.clone())),
        Ok(carrier) => Err(FaceFrameFault::NotPlanar { carrier }),
        Err(error) => Err(FaceFrameFault::Unresolved { error }),
    }
}

/// **Whether an evaluated value IS one body** — the evaluator's
/// single-body operand door (`eval::wire::body_operand`) asked of a
/// value the viewer is holding.
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
fn is_one_body(payload: &ValuePayload<f64>) -> bool {
    matches!(
        payload,
        ValuePayload::Body(_) | ValuePayload::Boolean(BooleanValue::Body { .. })
    )
}
