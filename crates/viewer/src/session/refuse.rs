//! **Why an operation did not happen**: the session's refusal
//! vocabulary, its ranking ladder, and the sentences the chrome shows
//! beside a refusal.
//!
//! A VOCABULARY. [`Refusal`] is a value, [`Refusal::rank`] orders two
//! of them and the wording composers are pure functions over their
//! arguments; nothing here names the session. [`NodeKindWanted`] and
//! [`admits`] live here because the kind a seat wants is a
//! [`Refusal::WrongNodeKind`] payload and `admits` is its predicate.

use pncad::document::{
    Datum, Dimension, DimensionError, DocumentId, EditError, Node, ParamName, ParseError,
    ProfileProgram, RecipeNodeId, SlotId,
};
use pncad::workspace::WorkspaceError;

use crate::combine;
use crate::display::DisplayFault;
use crate::docio::DocIoError;
use crate::props::{self, SlotValue};

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
        NodeKindWanted::Frame => matches!(held, Some(Node::Datum(Datum::Frame { .. }))),
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
    /// A boolean was authored with one node in both operand seats.
    ///
    /// The DAG admits it — an id in two input positions is neither a
    /// cycle nor a dangling reference — and the kernel would be asked
    /// to regularize a body against itself, whose answer is the body
    /// (or, for a subtraction, ∅) and whose faces are all coincident.
    /// It is a mis-pick every time, so the door says so rather than
    /// letting a degenerate operand pair reach the classifier. Two
    /// DIFFERENT nodes denoting the same geometry are not this
    /// refusal: it is a fact about the authored references, which is
    /// the only thing a door can be sure of.
    SelfBoolean {
        /// The node picked into both seats.
        node: RecipeNodeId,
    },
    /// `apply` refused the edit.
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
    /// A file operation failed.
    Io(Box<DocIoError>),
    /// Undo at the root, or redo at the tip of the current branch.
    NothingToDo,
    /// A display-state operation refused (hide on a non-instance, a
    /// free-move on a mate-constrained instance, a gesture out of
    /// order) — the fault's own typed vocabulary, unaltered.
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
            | Self::SelfBoolean { .. }
            | Self::Edit(_)
            | Self::Dimension(_)
            | Self::Parse(_)
            | Self::SlotUnit(_)
            | Self::NoDocumentDirectory
            | Self::Workspace(_)
            | Self::SelfInstance { .. }
            | Self::Io(_) => 1,
            // The two gesture-order arms rank with their document
            // twins; the substantive display refusals rank with the
            // real failures, because "this instance is mate-
            // constrained" is a decision about what the user tried.
            Self::Display(DisplayFault::NoFreeMove | DisplayFault::FreeMoveInFlight) => 2,
            Self::Display(_) => 1,
            Self::NoGesture | Self::GestureInFlight | Self::NothingToDo => 2,
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
    /// of the decision, so it is composed once and every surface that
    /// shows it — the status line, the inline note under the slot row —
    /// calls this. Two independently-built copies is how the wording
    /// drifts from the decision.
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
    pub fn exists_wording(name: &ParamName, dimension: Dimension) -> String {
        format!(
            "parameter {} already exists ({dimension:?}) — edit it instead?",
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
            Self::NoSuchParam(name) => write!(f, "no document parameter named {}", name.0),
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
            Self::SelfBoolean { node } => {
                write!(
                    f,
                    "a boolean needs two different bodies; node {} is in both operand seats",
                    node.0
                )
            }
            Self::Edit(error) => write!(f, "the edit was refused: {error}"),
            Self::Dimension(error) => write!(f, "{error}"),
            Self::Parse(error) => write!(f, "the expression did not parse: {error}"),
            Self::NoGesture => write!(f, "no drag is in progress"),
            Self::GestureInFlight => write!(f, "finish the drag first"),
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
        }
    }
}

impl core::error::Error for Refusal {}
