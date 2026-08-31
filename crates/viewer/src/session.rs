//! The document session: everything the panels operate on, and the
//! typed operations that operate on it.
//!
//! # One shape, from the toolkit and from a test
//!
//! G1's rule is that every operation the GUI performs is itself API.
//! [`SessionOp`] is that vocabulary for the panels, [`DocSession::perform`]
//! is the only function that performs one, and [`OpOutcome`] reports
//! what it emitted. A widget's job is to name an op; a test's job is to
//! name the same op and read the outcome. Nothing the panels can do is
//! expressible only as a click.
//!
//! # Preview and commit are the same edit, entered differently
//!
//! A continuous gesture — dragging a slider over a dimension —
//! evaluates PREVIEW edits against a scratch document and commits
//! exactly one `DocEdit` on release. The scratch document is a value
//! beside the history, never in it, so a gesture leaves no trace if it
//! is abandoned and exactly one undo step if it is not. Transient
//! gesture state (which slot, the value in flight) lives here in layer
//! 3 and never enters the document.
//!
//! # Every edit re-enters at the same door
//!
//! `apply` is pure, so a committed edit produces a new document value,
//! which is pushed onto the history and submitted to the evaluation
//! seam under a fresh generation. There is one such door
//! ([`DocSession::commit`]) and one submit ([`DocSession::request_eval`]);
//! undo, redo, open and every edit route through them, which is why
//! "the picture agrees with the document" is a property of the
//! structure rather than of each call site remembering.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use pncad::document::{
    Alignment, CancelToken, ChecksConfig, ChecksReport, Datum, Dimension, DimensionError, Doc,
    DocEdit, DocParam, DocRef, DocumentId, EditError, EvalOptions, Evaluation, Expr, Frame,
    LoopProgram, Node, ParamName, ParseError, PartResolver, ProductError, ProfileProgram,
    RecipeNodeId, SlotId, apply, assemble, cascade_delete_order, evaluate, parse_expr, product,
    run_checks,
};
use pncad::geom_core::Tol;
use pncad::prelude::{StableName, attribute};
use pncad::profile::SketchPlane;
use pncad::quantity::UnitDef;
use pncad::select::{ContactClass, Resolution, RunCtx, resolve};
use pncad::workspace::WorkspaceError;

use crate::bounds;
use crate::display::{DisplayFault, DisplayState, DisplayView};
use crate::docio::{self, DirResolver, DocIoError};
use crate::evalseam::{EvalRequest, EvalService, Generation, InlineEvaluator};
use crate::history::History;
use crate::parts;
use crate::props::{self, SlotDriver, SlotValue};
use crate::tree::{self, TreeRow};

/// A picked face: the stable name it is, and the node whose body
/// carried it when it was picked.
///
/// **The name is the selection**; the node rides along because it is
/// what the feature tree highlights and what the property panel shows
/// slots for, and re-deriving it would mean resolving the name again
/// for a question the pick already answered. G1's rule is satisfied
/// exactly: a `StableName` and a `RecipeNodeId`, no arena key.
///
/// The node is the one whose evaluated body was hit, which is not the
/// node that MADE the face: a face swept by an extrude, cut by a
/// boolean and carried through a fillet is hit on the fillet's body
/// and made by the extrude. Both are true and they answer different
/// questions — this field answers "whose body did the ray meet", and
/// [`FaceSelection::feature`] answers "which feature is this face's".
/// Every consumer that means the second must call it: on a model whose
/// history ends in one outer feature, this field is that feature for
/// every face of the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceSelection {
    /// The picked face's stable name — what survives re-evaluation.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
}

impl FaceSelection {
    /// **The feature this face is**: the node whose operation minted
    /// the entity the name denotes, read off the name's own
    /// carry-through segments (`pncad::select::attribute`).
    ///
    /// A fillet's `FromTarget(f)` face is still the target's face `f`,
    /// so clicking a flat on a filleted body reaches the feature that
    /// swept the flat and not the fillet that shrank it. That is the
    /// question the feature tree's highlight, the property panel's
    /// rows and the picture's focus all ask; [`FaceSelection::node`] —
    /// whose body the ray met — is a different one, and the only
    /// consumers that want it are the ones addressing the DRAWN body
    /// (`PickIndex::ids_of_target`, and the resolution check, which
    /// looks the name up in that body's own table).
    ///
    /// Falls back to [`FaceSelection::node`] for a name the vocabulary
    /// walk cannot classify, so an unclassified role degrades to the
    /// drawn root rather than to no feature at all.
    pub fn feature(&self) -> RecipeNodeId {
        attribute(&self.name).minted_by().unwrap_or(self.node)
    }
}

/// A picked edge: the stable name it is, and the node whose body
/// carried it when it was picked.
///
/// The face selection's twin, field for field, and deliberately a
/// DISTINCT type rather than a kind tag on one struct: the consumers
/// differ in what they accept — a blend selects edges, a mate selects
/// faces — so a value that could be either defers a refusal to run
/// time for no gain. The name is still the selection and no arena key
/// appears, which is all G1 asks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeSelection {
    /// The picked edge's stable name — what survives re-evaluation.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
}

impl EdgeSelection {
    /// **The feature this edge is**: the node whose operation minted
    /// the entity the name denotes — [`FaceSelection::feature`]'s
    /// argument, unchanged. An edge carried through a later boolean is
    /// still the edge the earlier feature made.
    pub fn feature(&self) -> RecipeNodeId {
        attribute(&self.name).minted_by().unwrap_or(self.node)
    }
}

/// What the cursor is over — the one transient pick, whichever kind of
/// entity it landed on.
///
/// One value rather than a field per kind, because the cursor is over
/// AT MOST ONE thing: two fields could both be set, and then the
/// picture and the status line would disagree about what the pointer
/// means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Hovered {
    /// A face under the cursor.
    Face(FaceSelection),
    /// An edge under the cursor — within
    /// [`crate::pick::EDGE_PICK_RADIUS_PX`] of it, which is what makes
    /// an edge reachable at all where its own face fills the pixel.
    Edge(EdgeSelection),
}

impl Hovered {
    /// The hovered entity's stable name.
    pub fn name(&self) -> &StableName {
        match self {
            Self::Face(face) => &face.name,
            Self::Edge(edge) => &edge.name,
        }
    }

    /// The node whose drawn body the cursor is over.
    pub fn node(&self) -> RecipeNodeId {
        match self {
            Self::Face(face) => face.node,
            Self::Edge(edge) => edge.node,
        }
    }

    /// The feature the hovered entity belongs to.
    pub fn feature(&self) -> RecipeNodeId {
        match self {
            Self::Face(face) => face.feature(),
            Self::Edge(edge) => edge.feature(),
        }
    }

    /// The hovered face, when the cursor is over one.
    pub fn face(&self) -> Option<&FaceSelection> {
        match self {
            Self::Face(face) => Some(face),
            Self::Edge(_) => None,
        }
    }

    /// The hovered edge, when the cursor is over one.
    pub fn edge(&self) -> Option<&EdgeSelection> {
        match self {
            Self::Edge(edge) => Some(edge),
            Self::Face(_) => None,
        }
    }

    /// This hover as the selection a click on it would make.
    pub fn selection(&self) -> Selection {
        match self {
            Self::Face(face) => Selection::Face(face.clone()),
            Self::Edge(edge) => Selection::Edge(edge.clone()),
        }
    }
}

/// What the session has selected. A typed layer-3 value: stable
/// names, recipe node ids and parameter names, never an arena key.
///
/// **Single-select, by ratification** (the GUI plan's rulings): one
/// selection, and nothing here is shaped to grow a second. Multi-select
/// is GQ7 and deferred by design.
///
/// **ONE value for the viewport and the panels.** A face picked in the
/// viewport and a node clicked in the tree write the same field, which
/// is what makes "click a face, watch its feature highlight" a
/// property of the state rather than of two widgets agreeing —
/// [`Selection::node`] is the one inversion both read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Selection {
    /// Nothing selected.
    #[default]
    None,
    /// A recipe node, selected in the feature tree.
    Node(RecipeNodeId),
    /// A document parameter, selected in the property panel — where
    /// the expression-driven refusal's affordance navigates to.
    Param(ParamName),
    /// A face, picked in the viewport.
    Face(FaceSelection),
    /// An edge, picked in the viewport — what a blend is authored
    /// against.
    Edge(EdgeSelection),
}

impl Selection {
    /// The recipe node this selection is about, when it is about one:
    /// the node itself, or the feature a picked face belongs to
    /// ([`FaceSelection::feature`] — the node that MADE the face, not
    /// the root that drew it).
    ///
    /// **The one home for the viewport→tree inversion.** The feature
    /// tree's highlight and the property panel's slot rows both read
    /// it, so a face pick reaches them without either of them knowing
    /// what a face is.
    pub fn node(&self) -> Option<RecipeNodeId> {
        match self {
            Self::Node(id) => Some(*id),
            Self::Face(face) => Some(face.feature()),
            Self::Edge(edge) => Some(edge.feature()),
            Self::None | Self::Param(_) => None,
        }
    }

    /// The picked face, when the selection is one.
    pub fn face(&self) -> Option<&FaceSelection> {
        match self {
            Self::Face(face) => Some(face),
            Self::None | Self::Node(_) | Self::Param(_) | Self::Edge(_) => None,
        }
    }

    /// The picked edge, when the selection is one.
    pub fn edge(&self) -> Option<&EdgeSelection> {
        match self {
            Self::Edge(edge) => Some(edge),
            Self::None | Self::Node(_) | Self::Param(_) | Self::Face(_) => None,
        }
    }

    /// The selected entity's stable name, when the selection is a
    /// picked entity — the one question the resolution check asks that
    /// does not care which kind was picked.
    pub fn entity_name(&self) -> Option<&StableName> {
        match self {
            Self::Face(face) => Some(&face.name),
            Self::Edge(edge) => Some(&edge.name),
            Self::None | Self::Node(_) | Self::Param(_) => None,
        }
    }
}

/// Whether the selection still denotes something in the evaluation on
/// screen — the ratified resolution-failure semantics, as a value.
///
/// **A vanished reference is a STATE, not an event.** Nothing clears
/// the selection when the thing it names stops existing: the name
/// stays, this verdict changes, and the chrome renders the unresolved
/// state distinctly while the affordances that need a live entity
/// switch off. That is the whole of GQ7's recorded constraint (tools
/// survive the referenced entity vanishing) at v1's single-select
/// scope.
#[derive(Clone, Debug, PartialEq)]
pub enum Standing {
    /// There is nothing selected to resolve.
    Empty,
    /// A node selection, and whether the document still holds it.
    Node {
        /// The node.
        node: RecipeNodeId,
        /// Whether it is still in the recipe.
        present: bool,
    },
    /// A parameter selection, and whether the document still declares
    /// it.
    Param {
        /// The parameter.
        name: ParamName,
        /// Whether it is still declared.
        present: bool,
    },
    /// A face selection, and the resolution verdict its name got.
    Face {
        /// The selection.
        face: FaceSelection,
        /// What the shipped resolution machinery answered — `None`
        /// when there is no evaluation to answer against yet, which is
        /// neither "live" nor "vanished" and is not reported as
        /// either.
        ///
        /// Boxed for the reason [`Refusal::Edit`] is: a `Resolution`
        /// carrying a diagnosis and a tombstone is an order of
        /// magnitude wider than the other arms here, and this value is
        /// returned by value on every frame.
        resolution: Option<Box<Resolution>>,
    },
    /// An edge selection, and the resolution verdict its name got.
    ///
    /// The same shape as [`Standing::Face`] because it is the same
    /// question asked of the same machinery: `resolve` takes a stable
    /// name and does not care which kind of entity minted it, so an
    /// edge selection survives its referent vanishing by exactly the
    /// face arm's rule rather than by a second implementation of it.
    Edge {
        /// The selection.
        edge: EdgeSelection,
        /// What the shipped resolution machinery answered — `None`
        /// when there is no evaluation to answer against yet.
        resolution: Option<Box<Resolution>>,
    },
}

impl Standing {
    /// Whether the selection denotes something the chrome may edit
    /// against.
    ///
    /// The one predicate the dependent affordances read. A face whose
    /// name did not resolve is NOT live; so is one with no evaluation
    /// behind it yet, because "we cannot tell" and "yes" are not the
    /// same answer and only one of them may enable a button.
    pub fn live(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Node { present, .. } | Self::Param { present, .. } => *present,
            Self::Face { resolution, .. } | Self::Edge { resolution, .. } => {
                matches!(resolution.as_deref(), Some(Resolution::Resolved(_)))
            }
        }
    }

    /// The typed unresolved verdict, when the selection has one.
    ///
    /// `Some` exactly when a picked entity's name failed to resolve or
    /// the evaluation could not answer for it — the two arms that
    /// render distinctly, for a face and for an edge alike.
    pub fn unresolved(&self) -> Option<&Resolution> {
        match self {
            Self::Face {
                resolution: Some(resolution),
                ..
            }
            | Self::Edge {
                resolution: Some(resolution),
                ..
            } if !matches!(**resolution, Resolution::Resolved(_)) => Some(resolution),
            _ => None,
        }
    }
}

/// The node kind a creation op's seat requires — the payload of
/// [`Refusal::WrongNodeKind`], so the refusal names what was wanted
/// in the vocabulary's own words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKindWanted {
    /// A `Node::Profile`.
    Profile,
    /// A `Node::Datum(Datum::Axis)`.
    Axis,
}

impl NodeKindWanted {
    /// The kind's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::Profile => "a profile",
            Self::Axis => "an axis datum",
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
    /// "replace" distinct ACTS — see [`SessionOp::CreateParam`]. The
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

impl Refusal {
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

/// **What the delete button says, and the list it says it about.**
///
/// A destructive action that understates itself is worse than one that
/// refuses: [`SessionOp::DeleteNode`] takes the target's whole
/// downstream cone, which in a chain-shaped model (a boolean per
/// feature) is most of the document, so the count belongs on the
/// button and not only in a tooltip.
///
/// The wording lives here rather than in the chrome for the reason
/// [`Refusal::exists_wording`] does: one composition, read by whatever
/// renders it, and testable without a window.
#[derive(Clone, Debug)]
pub struct DeleteAffordance {
    /// The button's label.
    pub label: String,
    /// The hover text; `None` when the label already says everything,
    /// which is exactly when nothing depends on the target.
    pub hover: Option<String>,
    /// The nodes the delete would remove, consumers first and the
    /// target last — the operation's own order.
    pub cascade: Vec<RecipeNodeId>,
}

impl DeleteAffordance {
    /// Compose the sentences for deleting `node` out of `doc`.
    ///
    /// The kind name is the node vocabulary's own (first-light finding
    /// #1097: reached from a face selection, a bare "Delete feature"
    /// read as deleting the *face* — an entity this vocabulary can
    /// never delete). The id-only arm is for a node the document does
    /// not hold: no button renders for one today, and if that changes
    /// the label stays honest rather than panicking.
    ///
    /// Neither sentence mentions the features that merely FED the
    /// target and survive as roots of their own, because this delete
    /// does not touch them. A delete that reconnected a target's
    /// consumers to its input instead — splice — is open as issue
    /// #1324.
    fn of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Self {
        let cascade = cascade_delete_order(doc, node);
        let Some(target) = doc.node(node) else {
            return Self {
                label: format!("Delete feature {}", node.0),
                hover: None,
                cascade,
            };
        };
        let kind = tree::node_kind(target);
        // The cascade's last entry is the target itself; everything
        // before it is a dependent.
        let dependents = cascade.len().saturating_sub(1);
        if dependents == 0 {
            return Self {
                label: format!("Delete feature '{kind}'"),
                hover: None,
                cascade,
            };
        }
        let (plural, depend) = if dependents == 1 {
            ("", "depends")
        } else {
            ("s", "depend")
        };
        let census = kind_census(doc, &cascade[..dependents]);
        Self {
            label: format!("Delete feature '{kind}' and {dependents} dependent feature{plural}"),
            hover: Some(format!(
                "Also deletes {dependents} feature{plural} that {depend} on it: {census}"
            )),
            cascade,
        }
    }
}

/// A census of node kinds, most numerous first and ties broken by
/// name, as `20 × Boolean, 1 × Fillet` — the readable form of a list
/// whose LENGTH is the thing being warned about.
fn kind_census(doc: &Doc<ProfileProgram>, nodes: &[RecipeNodeId]) -> String {
    let mut counts: std::collections::BTreeMap<&'static str, usize> =
        std::collections::BTreeMap::new();
    for &id in nodes {
        if let Some(node) = doc.node(id) {
            *counts.entry(tree::node_kind(node)).or_default() += 1;
        }
    }
    let mut census: Vec<(&'static str, usize)> = counts.into_iter().collect();
    census.sort_by_key(|&(kind, count)| (core::cmp::Reverse(count), kind));
    census
        .into_iter()
        .map(|(kind, count)| format!("{count} × {kind}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The literal payload of one add-datum form (GAUTH-1): plain numbers
/// in canonical units. The SESSION mints the `Expr` literals and
/// refuses a non-finite component typed
/// ([`Refusal::Dimension`]), so no form ever constructs an
/// expression — chrome deals in numbers, the vocabulary in values.
///
/// Component dimensions follow the [`Datum`] slots they land in:
/// origins and positions are Lengths, normals and directions are
/// Scalars (an unnormalized direction; evaluation normalizes or
/// refuses degenerate loudly).
#[derive(Clone, Copy, Debug)]
pub enum DatumSpec {
    /// A plane through `origin` with normal `normal`.
    Plane {
        /// Origin components, metres.
        origin: [f64; 3],
        /// Normal components, unitless.
        normal: [f64; 3],
    },
    /// An axis through `origin` along `direction`.
    Axis {
        /// Origin components, metres.
        origin: [f64; 3],
        /// Direction components, unitless.
        direction: [f64; 3],
    },
    /// A point at `position`.
    Point {
        /// Position components, metres.
        position: [f64; 3],
    },
}

/// One template loop of the add-profile door (GAUTH-1): the template
/// vocabulary, not a sketcher — the two forms the plan rules in,
/// spelled as plain numbers. The session lowers each to its
/// [`LoopProgram`] constructor and refuses a non-finite field typed;
/// a degenerate loop (zero radius, zero width) refuses through the
/// edit door's own authoring-time check, exactly as a hand-written
/// program would.
#[derive(Clone, Copy, Debug)]
pub enum ProfileShape {
    /// A circle ([`LoopProgram::circle`]).
    Circle {
        /// The centre, in sketch coordinates (metres).
        centre: [f64; 2],
        /// The radius, metres.
        radius: f64,
    },
    /// An axis-aligned rectangle centred on the sketch origin
    /// ([`LoopProgram::polygon`], corners at `(±w/2, ±h/2)`).
    Rectangle {
        /// The width (x extent), metres.
        width: f64,
        /// The height (y extent), metres.
        height: f64,
    },
}

/// One typed operation on the session.
#[derive(Clone, Debug)]
pub enum SessionOp {
    /// Move the selection.
    Select(Selection),
    /// Move the transient hover, or clear it with `None`.
    ///
    /// **Hover is layer-3 state and nothing else**: it never enters
    /// the document, never enters the history, and is not persisted.
    /// It is an operation rather than a field the chrome writes for
    /// the same reason every other move here is one (G1's
    /// operations-are-API rule) — a headless test hovers by naming
    /// this op.
    Hover(Option<Hovered>),
    /// Delete a recipe node **and every node downstream of it**, as
    /// one action and one undo.
    ///
    /// The panel's own door to `DocEdit::DeleteNode`, and the edit the
    /// survival semantics are exercised through: deleting the feature
    /// a selected face belongs to is the first of the three ways a
    /// selection's referent goes away.
    ///
    /// The cost is knowable before the click:
    /// [`DocSession::delete_affordance`] carries the same list the
    /// operation deletes. What it removes is the dependent CONE — a delete that
    /// reconnected consumers to the target's input instead (splice) is
    /// open as issue #1324.
    DeleteNode {
        /// The node to delete, with its dependents.
        node: RecipeNodeId,
    },
    /// Write a value into a node's slot. Refused if the slot is
    /// driven by an expression.
    SetSlot {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The new value.
        value: SlotValue,
    },
    /// Take a locally-valid-range probe for one field: how far it can
    /// move, in each direction, before the document acquires a failure
    /// it does not have now.
    ///
    /// **Explicitly asked for, never automatic.** The probe costs tens
    /// of evaluations (`bounds`'s cost note), which is a price worth
    /// paying when a user is about to change a number and not worth
    /// paying on every selection. The answer lands in the session
    /// ([`DocSession::bounds`]) and is discarded on the next document
    /// change.
    ///
    /// It changes no document, commits nothing and enters no history:
    /// every candidate is applied to a scratch copy that is dropped.
    ProbeBounds {
        /// The field to probe.
        target: BoundsTarget,
    },
    /// Change how a slot's literal is WRITTEN — its display unit —
    /// leaving the canonical value bit-identical.
    ///
    /// A separate door from [`SessionOp::SetSlot`] because the value
    /// and its notation are independent facts about a literal (D7 keeps
    /// the display unit out of expression identity entirely), and an
    /// operation that moved both could not move either alone. `None`
    /// means "remember no unit": the value renders canonically, which
    /// is what a literal authored without a suffix already does.
    ///
    /// It is a document edit and enters the history like any other:
    /// the unit is stored in the document and persists, so changing it
    /// is a change to the document, not to the picture.
    SetSlotUnit {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The unit to write it in, or `None` for canonical.
        unit: Option<UnitDef>,
    },
    /// Replace a slot's expression from source text, through the
    /// shipped `parse_expr` door. This is the affordance's editing
    /// half: a driven slot refuses a NUMBER, never an expression.
    SetSlotExpression {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
        /// The expression source.
        text: String,
    },
    /// Write a value into a document parameter.
    SetParam {
        /// The parameter.
        name: ParamName,
        /// The new value.
        value: SlotValue,
    },
    /// Declare a NEW document parameter — the panel's create
    /// affordance, committing exactly one `DocEdit::SetDocParam`.
    ///
    /// That edit is create-or-replace, and stays so at the API. This
    /// door refuses an already-declared name typed
    /// ([`Refusal::ParamExists`]): a "create" that replaced would
    /// change not just the value but possibly the declared DIMENSION,
    /// re-validating every referencing expression — a blast radius no
    /// plus-shaped button should carry. Replacing stays spellable
    /// through the door that says so ([`SessionOp::SetParam`], which
    /// conversely refuses a name that does NOT exist — the two doors
    /// partition the edit's semantics).
    CreateParam {
        /// The new parameter's name.
        name: ParamName,
        /// Its declared dimension and exact value.
        value: DocParam,
    },
    /// Start a continuous gesture over a slot.
    BeginGesture {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// Start a continuous gesture over a DOCUMENT PARAMETER.
    ///
    /// The same preview/commit machinery as [`SessionOp::BeginGesture`]
    /// and deliberately a separate door rather than a widened one: the
    /// two targets are addressed differently (a node and a slot; a
    /// name) and collapsing them would put an `Option` in every arm.
    /// A parameter is where the expression-driven affordance sends a
    /// user, so it is a dragged widget on a primary path and gets the
    /// gesture rule the ratified preview-vs-commit decision demands.
    BeginParamGesture {
        /// The parameter.
        name: ParamName,
    },
    /// Move the in-flight gesture. Emits a preview edit against
    /// scratch state; commits nothing.
    PreviewGesture {
        /// The value under the pointer.
        value: f64,
    },
    /// Release: commit exactly one edit carrying the gesture's last
    /// previewed value.
    CommitGesture,
    /// Abandon the gesture, leaving the document untouched.
    CancelGesture,
    /// Step the cursor toward the root.
    Undo,
    /// Step the cursor along the current branch.
    Redo,
    /// Cancel the evaluation in flight.
    CancelEvaluation,
    /// Ask for the current document to be evaluated again.
    ///
    /// The pair to [`SessionOp::CancelEvaluation`], and the only way
    /// back from a cancel: a canceled run leaves the picture older than
    /// the document with nothing running, and every other route to a
    /// re-submit goes through an edit or an undo, i.e. through changing
    /// the document to get it re-drawn. The memo makes the re-run of an
    /// unchanged document nearly free.
    Reevaluate,
    /// Replace the session's document with a file's.
    Open(PathBuf),
    /// Write the current path to a file.
    ///
    /// **Saving a COPY beside the original bricks the store for both**
    /// (issue #1117): the copy claims the same document id, so the
    /// directory then holds two files with one identity and every
    /// resolution through it refuses `DuplicateId` — typed and
    /// recoverable (delete either file), but a surprising blast
    /// radius for an ordinary act. The identity is the document's, not
    /// the file's, so a cheap rename-on-save cannot fix it without
    /// forking the document; the issue carries the design question.
    /// Save over the original, or save into a different directory.
    ///
    /// **A save-as MOVES the document seam**: the directory rule
    /// follows the file, so an assembly saved into a directory that
    /// does not hold its parts silently rebinds to that directory and
    /// its instances refuse at the next evaluation — typed on the tree
    /// badges, and recoverable by saving back, but nothing warns
    /// first. Recorded with the rest of the seam's newly-reachable
    /// edges in issue #1387.
    Save(PathBuf),
    /// Hide or show one instance (G3): a DISPLAY operation — the
    /// scene and the pick index drop a hidden instance; the document
    /// and the feature tree keep it, and nothing is persisted.
    SetInstanceHidden {
        /// The instance.
        instance: RecipeNodeId,
        /// Hidden, or shown again.
        hidden: bool,
    },
    /// Open a free-move probe gesture on a completely-unconstrained
    /// instance (G3's fit probe). Refused typed for a mate-
    /// constrained instance — eligibility is derived from the
    /// document, never guessed from solver state.
    BeginFreeMove {
        /// The instance to probe.
        instance: RecipeNodeId,
    },
    /// Stream the probe's display frame. Each preview REPLACES the
    /// last; nothing enters the document.
    PreviewFreeMove {
        /// The display frame composed over the instance's drawn
        /// placement.
        frame: Frame,
    },
    /// Land the probe: the last previewed frame becomes the
    /// instance's committed display value. NO history holds it — the
    /// plan's undo note governs document state only.
    CommitFreeMove,
    /// Abandon the probe, restoring the committed picture.
    CancelFreeMove,
    /// Commit **exactly one** `DocEdit` adding a mate node — the mate
    /// tool's single committed edit. Everything before it (the two
    /// picks, the class choice, the derived frames) is tool state; the
    /// document transition is this op alone, entering at the same
    /// commit door as every other edit: one apply, one history state,
    /// one re-evaluation, and the free-move supersession prune.
    AddMate {
        /// The `a` reference (instance-qualified).
        a: StableName,
        /// The `b` reference (instance-qualified).
        b: StableName,
        /// The declared contact class.
        class: ContactClass,
        /// The alignment datum (frames in each instance's own part
        /// coordinates).
        alignment: Alignment,
    },
    /// Replace the session's document with a fresh empty one (GAUTH-1's
    /// creation door zero) — the ONE creation op that is a
    /// session-state replacement rather than an `InsertNode`, because
    /// before it runs there is no document to insert into.
    ///
    /// The identity ruling (logged in `docs/GAUTH-LOG.md`): the id is
    /// authored at creation from the typed name —
    /// `DocumentId::derive(&name)`, the deterministic spelling the
    /// demo corpus uses — and never re-minted at save. Two documents
    /// derived from one name collide at WORKSPACE resolution, where
    /// the store's duplicate-id refusal is the fail-loud recourse.
    ///
    /// Everything session-scoped is cleared: path, history, selection,
    /// hover, display state and the resolver (a fresh document has no
    /// backing file, so its instantiate nodes refuse with the shipped
    /// no-resolver semantics until it is saved). Refused mid-gesture;
    /// a blank name refuses [`Refusal::EmptyName`].
    ///
    /// The name is TRIMMED before the id is derived, so `" ring "` and
    /// `"ring"` are one document by design — surrounding whitespace is
    /// a typing accident, not an identity a user could re-derive on
    /// purpose.
    NewDocument {
        /// The document's name; the id is derived from its trim.
        name: String,
    },
    /// Insert one datum node with literal slots — the add-datum forms'
    /// one committed edit each.
    AddDatum {
        /// The datum's literal payload.
        datum: DatumSpec,
    },
    /// Insert one profile node from template loops on a plane — the
    /// add-profile tool's one committed edit.
    ///
    /// `loops` is DESCRIPTION order, nothing more: which loop is the
    /// outer boundary and which are holes is decided by the profile
    /// layer's containment forest at replay
    /// (`profile::structure`), never by list position — a list
    /// written holes-first describes the same profile. Every refusal
    /// is the edit door's own: an empty list, a degenerate loop and a
    /// non-nested pair all refuse through the authoring-time check
    /// ([`Refusal::Edit`]), one rule for authored and hand-written
    /// programs alike (only a non-finite field refuses earlier, at
    /// the literal door). The plane is frozen `f64` placement data
    /// (the program's own placement struct — a snapshot, never a
    /// reference to the geometry it may have been derived from).
    AddProfile {
        /// The sketch-plane placement the profile is authored on.
        plane: SketchPlane<f64>,
        /// The template loops, in description order.
        loops: Vec<ProfileShape>,
    },
    /// Insert one extrude of an existing profile node — the extrude
    /// tool's one committed edit. A `profile` that is not a
    /// `Node::Profile` in this document refuses
    /// [`Refusal::WrongNodeKind`] at the door.
    ///
    /// A NEGATIVE distance is admitted deliberately and builds: it is
    /// an extrusion along the negative sketch normal, the same value
    /// the property panel can author into the slot afterwards, and
    /// the door does not narrow what the vocabulary means.
    AddExtrude {
        /// The profile node extruded.
        profile: RecipeNodeId,
        /// The extrusion distance, metres (a literal Length slot).
        distance: f64,
    },
    /// Insert one revolve of an existing profile node about an
    /// existing axis datum — the revolve tool's one committed edit.
    /// Either seat's wrong-kind pick refuses
    /// [`Refusal::WrongNodeKind`] at the door.
    AddRevolve {
        /// The profile node revolved.
        profile: RecipeNodeId,
        /// The `Datum::Axis` node revolved about.
        axis: RecipeNodeId,
        /// The sweep angle, radians (a literal Angle slot; the
        /// chrome's default is a full turn).
        angle: f64,
    },
    /// Commit **exactly one** `DocEdit` inserting an instance of
    /// another document — the assembly-authoring door, and the second
    /// insert door after the mate tool's.
    ///
    /// **The pin is minted HERE, not carried in.** The op names only
    /// which part (`id`); the version it pins is the store's content
    /// at the moment of the commit
    /// (`Workspace::current_pin`), so an authored reference always
    /// starts life resolving. From then on A4's Cargo.lock semantics
    /// hold: the pin moves by its own recorded edit and by nothing
    /// else.
    ///
    /// **The directory is the open file's own** — the same one every
    /// reference resolves against ([`crate::docio::DirResolver`]), so
    /// a session with no backing file refuses
    /// ([`Refusal::NoDocumentDirectory`]) rather than authoring a
    /// reference into a store it has not got.
    ///
    /// No placement is authored: A11 puts placement on the cluster and
    /// an instance carries no frame of its own, so the inserted node
    /// is complete with its reference and an empty interface record
    /// (an authored instance crosses no split seam). Hiding, the
    /// free-move probe and the mate tool take it from there.
    AddInstance {
        /// Which document in the open document's own directory.
        id: DocumentId,
    },
}

/// What an operation did.
#[derive(Debug, Default)]
pub struct OpOutcome {
    /// The edits that entered the history — at most one per op, and
    /// exactly one for a gesture's whole drag.
    pub committed: Vec<DocEdit<ProfileProgram>>,
    /// The edits evaluated against scratch state and NOT recorded.
    pub previewed: Vec<DocEdit<ProfileProgram>>,
    /// Why nothing (or nothing more) happened.
    pub refusal: Option<Refusal>,
    /// Instances whose free-move probe was **discarded** by this
    /// operation's document transition — the G3 supersession, reported
    /// rather than inferred: a mate landing on a probed instance
    /// removes its probe here, and the instance is drawn at its
    /// solved placement from the next landed evaluation on.
    pub superseded: Vec<RecipeNodeId>,
}

impl OpOutcome {
    fn refused(refusal: Refusal) -> Self {
        Self {
            refusal: Some(refusal),
            ..Self::default()
        }
    }
}

/// What a gesture is dragging.
#[derive(Clone, Debug)]
enum GestureTarget {
    /// A node's named slot, with the display unit that slot's literal
    /// remembered when the gesture opened.
    ///
    /// Captured at `begin_gesture` rather than re-read per preview
    /// because a gesture is defined against its BASE document (the one
    /// every preview is applied to), and the notation is a fact about
    /// that base. Re-reading it would read the scratch document the
    /// previews are writing — the gesture's own output.
    Slot {
        node: RecipeNodeId,
        slot: SlotId,
        unit: Option<UnitDef>,
    },
    /// A document parameter, with the dimension it is declared at.
    Param {
        name: ParamName,
        dimension: Dimension,
    },
}

impl GestureTarget {
    /// This target's dimension — a slot's from its `SlotId`, a
    /// parameter's from its declaration.
    fn dimension(&self) -> Dimension {
        match self {
            Self::Slot { slot, .. } => slot.dimension(),
            Self::Param { dimension, .. } => *dimension,
        }
    }

    /// Which arm a dragged `f64` becomes, through
    /// [`SlotValue::of`] — the one home for that rule.
    fn value_of(&self, value: f64) -> SlotValue {
        SlotValue::of(self.dimension(), value)
    }

    /// The edit that writes `value` into this target.
    ///
    /// A parameter's edit is the VALUE door, so the parameter's
    /// declaration — its dimension and any distribution — is read off
    /// the document by the edit itself rather than reassembled here.
    fn edit(&self, value: SlotValue) -> Result<DocEdit<ProfileProgram>, DimensionError> {
        match self {
            Self::Slot { node, slot, unit } => props::slot_edit(*node, *slot, value, *unit),
            Self::Param { name, .. } => Ok(props::param_edit(name.clone(), value)),
        }
    }
}

/// A gesture in flight: layer-3 state only.
#[derive(Debug)]
struct Gesture {
    target: GestureTarget,
    /// The document the previews are applied to — the history's
    /// current value, held so each preview replaces the last rather
    /// than stacking.
    base: Doc<ProfileProgram>,
    /// The last previewed value, and the one the commit records.
    value: Option<SlotValue>,
}

/// The whole of what the document panels operate on.
pub struct DocSession {
    history: History,
    tol: Tol,
    selection: Selection,
    /// What the cursor is over: transient, never persisted, and its
    /// ONE home. A widget that kept its own copy would be the
    /// per-widget shadow the panels' inventory discipline forbids.
    hover: Option<Hovered>,
    gesture: Option<Gesture>,
    scratch: Option<Doc<ProfileProgram>>,
    eval: Box<dyn EvalService>,
    generation: Generation,
    /// The document handed to the seam under [`DocSession::generation`]
    /// — kept so a landed result can be paired with the document it
    /// actually answers.
    ///
    /// `Arc` so that landing it costs a handle rather than a copy: the
    /// landed pair and the outstanding request name the SAME document
    /// value for as long as they agree, which is most of the time.
    requested_doc: Arc<Doc<ProfileProgram>>,
    landed: Option<Arc<Evaluation<f64>>>,
    /// The document [`DocSession::landed`] answers.
    ///
    /// **Resolution is a question about a PAIR.** `resolve` reads the
    /// recipe and the evaluation together, so asking it about the
    /// document as it stands now and the evaluation as it stood one
    /// edit ago is asking about a run that never happened — and the
    /// diagnosis it answers with would be about that non-run. The two
    /// move together here, one generation behind the shown document
    /// while a run is outstanding, which is exactly what the picture
    /// does.
    landed_doc: Option<Arc<Doc<ProfileProgram>>>,
    landed_generation: Option<Generation>,
    /// The gather's refusal for the landed pair, computed once when it
    /// lands ([`DocSession::product_fault`]).
    landed_fault: Option<ProductError>,
    /// The A5 at-rest verdict for the landed pair, computed once when
    /// it lands ([`DocSession::at_rest`]); `None` for a document that
    /// is not assembly-shaped, or before anything lands.
    landed_at_rest: Option<AtRestBadge>,
    /// The advisory-check report for the landed pair, computed once
    /// when it lands ([`DocSession::checks`]); `None` before anything
    /// lands, or when the registry itself refused.
    landed_checks: Option<ChecksReport>,
    path: Option<PathBuf>,
    /// Layer-3 display state — hide and free-move — with its one home
    /// here (the seam-friction inventory rule). Never persisted; reset
    /// by `Open`; pruned against every new document value.
    display: DisplayState,
    /// The document seam: the opened file's own directory, consulted
    /// lazily (the directory rule and the scan-at-resolution posture
    /// are [`DirResolver`]'s docs). A session over an in-memory
    /// document carries no resolver, and its instantiate nodes refuse
    /// typed. Replaced — never inherited — on every `Open`, so a
    /// document can never silently resolve against the previous
    /// document's directory.
    resolver: Option<Arc<DirResolver>>,
    /// The last locally-valid-range probe, and the field it was taken
    /// for — layer-3 state, never persisted, and its one home.
    ///
    /// Kept rather than recomputed per frame because a probe costs tens
    /// of evaluations: it is taken when asked for
    /// ([`SessionOp::ProbeBounds`]) and DISCARDED, never repaired, the
    /// moment the document changes. A range is a statement about one
    /// document, and showing yesterday's range beside today's number is
    /// the class of stale-confident answer this crate's staleness rules
    /// exist to prevent.
    bounds: Option<(BoundsTarget, bounds::Bounds)>,
}

/// The field a locally-valid-range probe was taken for.
///
/// Two arms rather than one with an `Option`, for the reason
/// `BeginGesture` and `BeginParamGesture` are two doors: a slot and a
/// document parameter are addressed differently, and collapsing them
/// puts an `Option` in every arm that reads one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundsTarget {
    /// A node's named slot.
    Slot {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// A document parameter.
    Param {
        /// The parameter.
        name: ParamName,
    },
}

/// The A5 at-rest verdict for the landed pair — a mated document's
/// declarations run through the kernel's own verification door
/// (`assemble`), once per landed evaluation, so a committed mate's
/// class verdict does not die at the commit: a `Tangent` that solves
/// green still shows the gate refusing it, and an undeclared contact
/// between instances surfaces on the draw path instead of waiting for
/// an export.
///
/// Taken only for assembly-shaped documents (one holding at least one
/// `InstantiatePart`) — a part document's tiers are not this badge's
/// subject, and the gate's cost is not spent where it answers nothing
/// the badges do not already say.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AtRestBadge {
    /// The gate certified the assembled product; how many declarations
    /// its mates minted.
    Certified {
        /// The minted declaration count.
        minted: usize,
    },
    /// The gate refused — its own rendering, never a sentence composed
    /// here.
    Refused {
        /// The typed refusal's `Display`.
        message: String,
    },
}

/// What happened to a result the seam handed back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Landing {
    /// It answered the current document and is now the session's.
    Landed,
    /// It answered an older request and was discarded.
    Stale,
    /// It answered the current request but was CANCELED, so it carries
    /// the completed prefix of a run nobody asked to see half of. The
    /// last good evaluation stays on screen and the session goes on
    /// owing an answer.
    Canceled,
}

impl DocSession {
    /// A session over `doc`, evaluated through `eval`.
    ///
    /// The first evaluation is requested here, so a session is never
    /// in a state where the document and the picture disagree because
    /// nobody asked.
    pub fn new(doc: Doc<ProfileProgram>, tol: Tol, eval: Box<dyn EvalService>) -> Self {
        let mut session = Self {
            // A placeholder the `request_eval` below overwrites before
            // anything can read it. It is the empty document rather
            // than a clone of `doc` because a clone here would be a
            // second copy nobody ever looks at.
            requested_doc: Arc::new(Doc::empty_derived("unsubmitted", tol)),
            history: History::new(doc),
            tol,
            selection: Selection::None,
            hover: None,
            gesture: None,
            scratch: None,
            eval,
            generation: Generation::FIRST,
            landed: None,
            landed_doc: None,
            landed_fault: None,
            landed_at_rest: None,
            landed_checks: None,
            landed_generation: None,
            path: None,
            display: DisplayState::new(),
            resolver: None,
            bounds: None,
        };
        session.request_eval();
        session
    }

    /// A session running the seam inline — the shape a test and the
    /// wasm build use.
    pub fn inline(doc: Doc<ProfileProgram>, tol: Tol) -> Self {
        Self::new(doc, tol, Box::new(InlineEvaluator::new()))
    }

    /// The document the panels show: the gesture's scratch value while
    /// one is in flight, the history's current value otherwise.
    pub fn doc(&self) -> &Doc<ProfileProgram> {
        self.scratch.as_ref().unwrap_or_else(|| self.history.doc())
    }

    /// The document the history is on, ignoring any preview.
    pub fn committed_doc(&self) -> &Doc<ProfileProgram> {
        self.history.doc()
    }

    /// The edit history.
    pub fn history(&self) -> &History {
        &self.history
    }

    /// **What a delete of `node` would cost, said before it is paid.**
    ///
    /// The `cascade` field is the deletion order itself — exactly what
    /// [`SessionOp::DeleteNode`] applies — so the number on the button
    /// and the number of features that vanish are one list read twice.
    ///
    /// Read off the COMMITTED document, which is the one the edits
    /// apply to; the delete op refuses outright while a gesture holds
    /// a scratch value, so the two never disagree at a live button.
    pub fn delete_affordance(&self, node: RecipeNodeId) -> DeleteAffordance {
        DeleteAffordance::of(self.committed_doc(), node)
    }

    /// The ε this session decides at.
    pub fn tol(&self) -> Tol {
        self.tol
    }

    /// The current selection.
    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    /// What the cursor is over, if anything.
    pub fn hover(&self) -> Option<&Hovered> {
        self.hover.as_ref()
    }

    /// Whether the selection still denotes something in the evaluation
    /// on screen.
    ///
    /// **A pure function of (shown document, landed evaluation,
    /// selection)** — recomputed, never cached, so it cannot be stale
    /// with respect to the state it describes. A face's verdict comes
    /// from the shipped `resolve` door; nothing here re-implements the
    /// resolution ladder or interprets its answer beyond arranging it
    /// beside the other two selection kinds.
    pub fn standing(&self) -> Standing {
        match &self.selection {
            Selection::None => Standing::Empty,
            Selection::Node(node) => Standing::Node {
                node: *node,
                present: self.doc().node(*node).is_some(),
            },
            Selection::Param(name) => Standing::Param {
                name: name.clone(),
                present: self.doc().params().contains_key(name),
            },
            Selection::Face(face) => Standing::Face {
                face: face.clone(),
                resolution: self.entity_resolution(&face.name),
            },
            Selection::Edge(edge) => Standing::Edge {
                edge: edge.clone(),
                resolution: self.entity_resolution(&edge.name),
            },
        }
    }

    /// One picked name's verdict against the landed run — the shipped
    /// `resolve` door, asked once and spelled once for both entity
    /// kinds.
    fn entity_resolution(&self, name: &StableName) -> Option<Box<Resolution>> {
        self.landed_pair()
            .map(|(doc, eval)| Box::new(resolve(RunCtx { doc, eval }, name)))
    }

    /// The most recent evaluation that answered the current document.
    pub fn evaluation(&self) -> Option<&Evaluation<f64>> {
        self.landed.as_deref()
    }

    /// The most recent evaluation, shared.
    pub fn evaluation_arc(&self) -> Option<&Arc<Evaluation<f64>>> {
        self.landed.as_ref()
    }

    /// The landed evaluation together with the document it answers —
    /// the pair every name question is asked of.
    ///
    /// The two fields are set in one place and read together, so a
    /// caller cannot pick up one without the other.
    pub fn landed_pair(&self) -> Option<(&Doc<ProfileProgram>, &Evaluation<f64>)> {
        Some((self.landed_doc.as_deref()?, self.landed.as_deref()?))
    }

    /// Why the landed evaluation's product does not gather, if it does
    /// not — the gather-level refusal no per-node badge can carry.
    ///
    /// `None` both when the product is well formed and when nothing
    /// has landed yet; [`DocSession::landed_pair`] distinguishes those.
    pub fn product_fault(&self) -> Option<&ProductError> {
        self.landed_fault.as_ref()
    }

    /// The A5 at-rest verdict for the landed pair ([`AtRestBadge`]),
    /// when the landed document is assembly-shaped. `None` for a part
    /// document, and before anything lands.
    pub fn at_rest(&self) -> Option<&AtRestBadge> {
        self.landed_at_rest.as_ref()
    }

    /// The last locally-valid-range probe, with the field it was taken
    /// for. `None` before any probe, and after every document change
    /// (`request_eval`'s discard).
    pub fn bounds(&self) -> Option<&(BoundsTarget, bounds::Bounds)> {
        self.bounds.as_ref()
    }

    /// The advisory-check report for the landed pair — findings in
    /// deterministic order, and the residents that were configured
    /// `Off`. `None` before anything lands, or when the registry
    /// refused (which is distinct from an empty report: "not checked"
    /// and "checked and fine" are different answers).
    pub fn checks(&self) -> Option<&ChecksReport> {
        self.landed_checks.as_ref()
    }

    /// The file this session is backed by, if any.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// The layer-3 display state (hide, free-move).
    pub fn display(&self) -> &DisplayState {
        &self.display
    }

    /// The display snapshot the scene and pick paths consume.
    pub fn display_view(&self) -> DisplayView {
        self.display.view(self.doc())
    }

    /// The directory this session resolves part references against —
    /// the opened file's own, or `None` for an in-memory document
    /// (the [`DirResolver`] directory rule).
    pub fn resolve_dir(&self) -> Option<&Path> {
        self.resolver.as_deref().map(DirResolver::dir)
    }

    /// The generation the session is waiting for a result on.
    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// The generation of the result currently on screen — what a
    /// consumer derived from the evaluation (the scene) compares
    /// against to know whether its own copy is current.
    pub fn landed_generation(&self) -> Option<Generation> {
        self.landed_generation
    }

    /// Whether the result on screen answers a document the session has
    /// already moved past — what the busy indicator reads. A VALUE the
    /// chrome consumes, not a spinner the seam draws.
    ///
    /// Defined against the SESSION's own two generations rather than
    /// by asking the seam whether it has work: those agree whenever
    /// the seam is the only source of results, and where they differ
    /// this is the honest one. "Am I showing the current document" is
    /// the question a busy indicator answers.
    pub fn busy(&self) -> bool {
        self.landed_generation != Some(self.generation)
    }

    /// Whether the seam actually has work outstanding.
    ///
    /// [`DocSession::busy`] asks "is the picture older than the
    /// document"; this asks "is anyone doing something about it". They
    /// agree except in one state, and that state is the whole reason
    /// this exists: after a Cancel the picture is older and **nothing
    /// is running**, so a chrome that only read `busy` would show a
    /// spinner over an idle seam forever. `busy() && !running()` is
    /// "canceled, showing an older result" — what
    /// [`SessionOp::Reevaluate`] recovers from.
    pub fn running(&self) -> bool {
        self.eval.busy()
    }

    /// The feature tree's rows for the shown document.
    pub fn tree_rows(&self) -> Vec<TreeRow> {
        // **The landed PAIR, not the shown document against the landed
        // evaluation.** A row's badge is a statement about what a run
        // said about a node, so reading it off a document that run
        // never saw describes a run that never happened — the same
        // defect `landed_pair` exists to make unreachable, and it lived
        // one function away from the fix that introduced it. While a
        // run is outstanding the tree therefore shows the picture's
        // document, which is what the viewport shows too.
        match self.landed_pair() {
            Some((doc, eval)) => tree::rows(doc, Some(eval)),
            // Nothing has landed: the shown document with no
            // evaluation, which renders every row `Unevaluated`.
            None => tree::rows(self.doc(), None),
        }
    }

    /// The property rows for the selected node — the node itself, or a
    /// picked face's owning node ([`Selection::node`]); empty for any
    /// other selection.
    ///
    /// **A selection whose referent has vanished offers no rows.**
    /// That is the "disables dependent affordances" half of the
    /// resolution-failure semantics, taken at the source rather than
    /// at each widget: a panel that cannot get rows cannot offer an
    /// edit against a face that is no longer there.
    pub fn slot_rows(&self) -> Vec<props::SlotRow> {
        if !self.standing().live() {
            return Vec::new();
        }
        match self.selection.node() {
            Some(id) => props::slot_rows(self.doc(), id),
            None => Vec::new(),
        }
    }

    /// The property rows as the panel LAYS THEM OUT — [`Self::slot_rows`]
    /// folded so that the three components of a 3-vector arrive as one
    /// group (`props::group_rows`).
    ///
    /// A second door rather than a replacement because the two answer
    /// different questions: a test asserting what a node's slots are
    /// wants the flat list, and a panel deciding how many lines to draw
    /// wants this. Both are the same rows — the grouping only bundles.
    pub fn slot_groups(&self) -> Vec<props::SlotGroup> {
        props::group_rows(self.slot_rows())
    }

    /// Take whatever the seam has finished, discarding results for
    /// documents the session has moved past.
    ///
    /// Returns one entry per result handled, so a caller can assert on
    /// what was discarded rather than infer it.
    pub fn pump(&mut self) -> Vec<Landing> {
        let mut landings = Vec::new();
        while let Some(done) = self.eval.poll() {
            landings.push(self.land(done));
        }
        landings
    }

    /// Decide one result's fate. Public so the staleness rule is
    /// testable without a scheduler.
    ///
    /// **Two filters, and both are refusals to show a wrong picture.**
    /// A result for a superseded request is [`Landing::Stale`]. A
    /// result for the CURRENT request that did not complete is
    /// [`Landing::Canceled`]: a canceled run answers with the prefix it
    /// finished before the token was seen, and that prefix is not the
    /// document — rendered, it is a feature tree of `Unevaluated` rows
    /// and a product that gathers to nothing. The last good evaluation
    /// therefore stays, and [`DocSession::busy`] goes on reporting that
    /// the picture is older than the document, which it is.
    pub fn land(&mut self, done: crate::evalseam::EvalDone) -> Landing {
        if done.generation != self.generation {
            return Landing::Stale;
        }
        if !done.completed() {
            return Landing::Canceled;
        }
        self.landed_generation = Some(done.generation);
        // **The product's own verdict, once per landed evaluation.**
        // The gather is the only thing that answers "is this
        // document's product well formed" — a naming collision across
        // roots is not a node failure, so the feature tree's badges
        // cannot see it, and a viewport that draws the parts without
        // ever asking would render a body nothing says is wrong.
        // Computed HERE because here is the one place a result becomes
        // the session's, so it cannot be run twice or skipped.
        self.landed_fault = product(self.requested_doc.as_ref(), &done.evaluation, self.tol).err();
        // The A5 verdict, in the same once-per-landing spot and for
        // the same reason: nowhere else can it run exactly once per
        // result that becomes the session's.
        self.landed_at_rest = at_rest_of(self.requested_doc.as_ref(), &done.evaluation, self.tol);
        // The advisory registry, same spot and same reason. It REPORTS
        // — a document with findings still draws, which is the whole
        // point of running it on the draw path: a product whose roots
        // interpenetrate renders a picture that looks almost right,
        // and the finding is the only thing that says otherwise.
        // A refusal of the registry itself leaves no report rather
        // than a clean one: "not checked" is not "checked and fine".
        self.landed_checks = run_checks(
            self.requested_doc.as_ref(),
            &done.evaluation,
            &ChecksConfig::default(),
            self.tol,
        )
        .ok();
        self.landed = Some(done.evaluation);
        self.landed_doc = Some(Arc::clone(&self.requested_doc));
        Landing::Landed
    }

    /// Perform one operation.
    pub fn perform(&mut self, op: SessionOp) -> OpOutcome {
        match op {
            SessionOp::Select(selection) => {
                self.selection = selection;
                OpOutcome::default()
            }
            SessionOp::Hover(hover) => {
                self.hover = hover;
                OpOutcome::default()
            }
            SessionOp::DeleteNode { node } => {
                if self.gesture.is_some() {
                    return OpOutcome::refused(Refusal::GestureInFlight);
                }
                self.delete_node(node)
            }
            SessionOp::SetSlot { node, slot, value } => self.set_slot(node, slot, value),
            SessionOp::ProbeBounds { target } => self.probe_bounds(target),
            SessionOp::SetSlotUnit { node, slot, unit } => self.set_slot_unit(node, slot, unit),
            SessionOp::SetSlotExpression { node, slot, text } => {
                self.set_slot_expression(node, slot, &text)
            }
            SessionOp::SetParam { name, value } => self.set_param(&name, value),
            SessionOp::CreateParam { name, value } => self.create_param(name, value),
            SessionOp::BeginGesture { node, slot } => self.begin_gesture(node, slot),
            SessionOp::BeginParamGesture { name } => self.begin_param_gesture(&name),
            SessionOp::PreviewGesture { value } => self.preview_gesture(value),
            SessionOp::CommitGesture => self.commit_gesture(),
            SessionOp::CancelGesture => {
                let had = self.gesture.take().is_some();
                // Same rule as a no-move commit: only a gesture that
                // actually put a scratch document on screen owes a
                // re-submit to take it away again.
                let previewed = self.scratch.take().is_some();
                if had {
                    if previewed {
                        self.request_eval();
                    }
                    OpOutcome::default()
                } else {
                    OpOutcome::refused(Refusal::NoGesture)
                }
            }
            SessionOp::Undo => self.step(true),
            SessionOp::Redo => self.step(false),
            SessionOp::CancelEvaluation => {
                self.eval.cancel();
                OpOutcome::default()
            }
            SessionOp::Reevaluate => {
                self.request_eval();
                OpOutcome::default()
            }
            SessionOp::Open(path) => self.open(&path),
            SessionOp::Save(path) => self.save(&path),
            SessionOp::SetInstanceHidden { instance, hidden } => {
                match self
                    .display
                    .set_hidden(self.history.doc(), instance, hidden)
                {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::BeginFreeMove { instance } => {
                match self.display.begin_free_move(self.history.doc(), instance) {
                    Ok(()) => OpOutcome::default(),
                    Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
                }
            }
            SessionOp::PreviewFreeMove { frame } => match self.display.preview_free_move(frame) {
                Ok(()) => OpOutcome::default(),
                Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
            },
            SessionOp::CommitFreeMove => match self.display.commit_free_move() {
                Ok(()) => OpOutcome::default(),
                Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
            },
            SessionOp::CancelFreeMove => match self.display.cancel_free_move() {
                Ok(()) => OpOutcome::default(),
                Err(fault) => OpOutcome::refused(Refusal::Display(fault)),
            },
            SessionOp::AddMate {
                a,
                b,
                class,
                alignment,
            } => {
                if self.gesture.is_some() {
                    return OpOutcome::refused(Refusal::GestureInFlight);
                }
                self.commit(DocEdit::InsertNode {
                    node: Node::Mate {
                        a,
                        b,
                        class,
                        alignment,
                    },
                })
            }
            SessionOp::NewDocument { name } => self.new_document(&name),
            SessionOp::AddDatum { datum } => self.add_datum(datum),
            SessionOp::AddProfile { plane, loops } => self.add_profile(plane, &loops),
            SessionOp::AddExtrude { profile, distance } => self.add_extrude(profile, distance),
            SessionOp::AddRevolve {
                profile,
                axis,
                angle,
            } => self.add_revolve(profile, axis, angle),
            SessionOp::AddInstance { id } => {
                if self.gesture.is_some() {
                    return OpOutcome::refused(Refusal::GestureInFlight);
                }
                self.add_instance(id)
            }
        }
    }

    /// The documents the open document's own directory offers as
    /// parts — the `Add part…` chooser's listing, as a value.
    ///
    /// # Errors
    ///
    /// [`Refusal::NoDocumentDirectory`] for a session with no backing
    /// file (there is no store to list), and [`Refusal::Workspace`]
    /// carrying the scan's own refusal — which is where a duplicate id
    /// or an unreadable sibling surfaces, at the chooser rather than
    /// at a tree badge, since no node exists yet to badge.
    pub fn part_catalogue(&self) -> Result<Vec<parts::PartEntry>, Refusal> {
        let resolver = self
            .resolver
            .as_deref()
            .ok_or(Refusal::NoDocumentDirectory)?;
        parts::catalogue(resolver, self.committed_doc().id())
            .map_err(|error| Refusal::Workspace(Box::new(error)))
    }

    /// Insert an instance of the part `id` names, minting its
    /// reference through the store: identity as asked for, version
    /// from the directory's content NOW.
    ///
    /// The store is reached through the resolver, which is the
    /// directory rule's home ([`DirResolver::workspace`]) — the same
    /// object every resolution consults, so the door a reference is
    /// authored through and the door it is later resolved through
    /// cannot come apart.
    ///
    /// The pin read is a full load of the referenced file (the store's
    /// own door), so a part that does not load refuses HERE — before a
    /// node exists — rather than as an unresolvable instance the user
    /// then has to delete.
    ///
    /// **Identity is checked before the directory**, deliberately: a
    /// document is its own document wherever its file lives, so the
    /// self-instance refusal is the one that survives saving, and
    /// naming the recoverable problem first would send a user off to
    /// save for nothing.
    fn add_instance(&mut self, id: DocumentId) -> OpOutcome {
        if let Some(refusal) = Refusal::self_instance(self.committed_doc().id(), id) {
            return OpOutcome::refused(refusal);
        }
        let Some(resolver) = self.resolver.as_deref() else {
            return OpOutcome::refused(Refusal::NoDocumentDirectory);
        };
        let pin = match resolver
            .workspace()
            .and_then(|ws| ws.current_pin(id, self.tol))
        {
            Ok(pin) => pin,
            Err(error) => return OpOutcome::refused(Refusal::Workspace(Box::new(error))),
        };
        self.commit(DocEdit::InsertNode {
            node: Node::instantiate_part(DocRef { id, pin }),
        })
    }

    /// The slot's driver and current value, or the refusal that says
    /// the slot is not there.
    fn driver_of(
        &self,
        node: RecipeNodeId,
        slot: SlotId,
    ) -> Result<(SlotDriver, Option<SlotValue>), Refusal> {
        let row = props::slot_rows(self.committed_doc(), node)
            .into_iter()
            .find(|row| row.slot == slot)
            .ok_or(Refusal::NoSuchSlot { node, slot })?;
        Ok((row.driver, row.value.ok()))
    }

    /// Refuse a numeric edit to a driven slot, with the affordance.
    fn guard_driven(&self, node: RecipeNodeId, slot: SlotId) -> Result<(), Refusal> {
        let (driver, current) = self.driver_of(node, slot)?;
        match driver {
            SlotDriver::Literal => Ok(()),
            SlotDriver::Expression { params } => Err(Refusal::DrivenByExpression {
                node,
                slot,
                params,
                current,
            }),
        }
    }

    fn set_slot(&mut self, node: RecipeNodeId, slot: SlotId, value: SlotValue) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if let Err(refusal) = self.guard_driven(node, slot) {
            return OpOutcome::refused(refusal);
        }
        let unit = props::slot_unit(self.committed_doc(), node, slot);
        match props::slot_edit(node, slot, value, unit) {
            Ok(edit) => self.commit(edit),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// Run one locally-valid-range probe, inline.
    ///
    /// The oracle is the whole of what "valid" means here: apply the
    /// candidate value to the shown document, evaluate, and ask whether
    /// the failing set grew ([`bounds::Verdict::no_worse_than`]). An
    /// edit the door itself REFUSES — a profile program that stops
    /// being a legal walk, a non-finite literal — counts as invalid
    /// without an evaluation, which is the same answer for the same
    /// reason: at that value the document does not stand.
    ///
    /// Every sample runs against the landed evaluation as its memo, so
    /// it re-runs the edited node's downstream cone rather than the
    /// whole recipe.
    fn probe_bounds(&mut self, target: BoundsTarget) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let base = self.doc().clone();
        let (origin, seed, integral) = match self.probe_scale(&target) {
            Ok(scale) => scale,
            Err(refusal) => return OpOutcome::refused(refusal),
        };
        let tol = self.tol;
        let prior = self.landed.clone();
        let resolver = self
            .resolver
            .as_ref()
            .map(|ws| Arc::clone(ws) as Arc<dyn PartResolver>);
        // The baseline is taken at the value the field HAS, from the
        // same oracle every sample goes through — so "no worse than the
        // baseline" compares two runs of one function rather than a run
        // against the landed evaluation, which may have been taken at a
        // different memo state.
        let baseline = bounds::Verdict::of(&evaluate_with(&base, prior.as_deref(), &resolver, tol));
        let result = bounds::probe(
            bounds::BoundsProbe::new(origin, seed, integral),
            |candidate| {
                let Some(edit) = Self::probe_edit(&base, &target, candidate) else {
                    return false;
                };
                match apply(&base, &edit, tol) {
                    Ok(applied) => {
                        let eval = evaluate_with(&applied.doc, prior.as_deref(), &resolver, tol);
                        bounds::Verdict::of(&eval).no_worse_than(&baseline)
                    }
                    // The edit door refused: at this value there is no
                    // document to evaluate, which is as invalid as a
                    // value gets.
                    Err(_) => false,
                }
            },
        );
        self.bounds = Some((target, result));
        OpOutcome::default()
    }

    /// The probe's three inputs, read off the field: where it is now,
    /// the step to search by, and whether its answer is an integer.
    ///
    /// The seed is ONE of whatever unit the field is written in — one
    /// millimetre for a slot written in millimetres, one radian for one
    /// written canonically, 1 for a count or a bare scalar. That is the
    /// scale a user thinks in, which is the scale a range should be
    /// searched and reported at.
    fn probe_scale(&self, target: &BoundsTarget) -> Result<(f64, f64, bool), Refusal> {
        match target {
            BoundsTarget::Slot { node, slot } => {
                let rows = props::slot_rows(self.doc(), *node);
                // One refusal for both misses — the node does not carry
                // the slot, and the slot carries no readable value —
                // because a probe needs a place to search FROM and
                // neither case gives it one.
                let found = rows
                    .into_iter()
                    .find(|row| row.slot == *slot)
                    .and_then(|row| Some((row.value.ok()?, row.dimension, row.unit)));
                let Some((value, dimension, remembered)) = found else {
                    return Err(Refusal::NoSuchSlot {
                        node: *node,
                        slot: *slot,
                    });
                };
                let value = value.as_f64();
                let unit = props::written_unit(dimension, remembered);
                Ok((
                    value,
                    props::from_written(1.0, unit),
                    dimension == Dimension::Count,
                ))
            }
            BoundsTarget::Param { name } => {
                let Some(param) = self.committed_doc().params().get(name) else {
                    return Err(Refusal::NoSuchParam(name.clone()));
                };
                let value = match param {
                    DocParam::Continuous { value, .. } => *value,
                    DocParam::Count { value } => *value as f64,
                };
                // A parameter stores no display unit (`props`' module
                // docs name the asymmetry), so its seed is one CANONICAL
                // unit.
                Ok((value, 1.0, param.dim() == Dimension::Count))
            }
        }
    }

    /// The edit that puts `value` into the probed field, or `None` when
    /// the value cannot be expressed there at all.
    fn probe_edit(
        doc: &Doc<ProfileProgram>,
        target: &BoundsTarget,
        value: f64,
    ) -> Option<DocEdit<ProfileProgram>> {
        match target {
            BoundsTarget::Slot { node, slot } => props::slot_edit(
                *node,
                *slot,
                SlotValue::of(slot.dimension(), value),
                props::slot_unit(doc, *node, *slot),
            )
            .ok(),
            BoundsTarget::Param { name } => {
                // The dimension is read off the DECLARATION only to
                // decide which `SlotValue` arm the sample becomes; the
                // edit itself carries a value and nothing else, so a
                // probe cannot disturb the parameter's declaration
                // (`props::param_edit`'s door).
                let dimension = doc.params().get(name)?.dim();
                Some(props::param_edit(
                    name.clone(),
                    SlotValue::of(dimension, value),
                ))
            }
        }
    }

    /// Rewrite a slot literal's display unit — the value stays put.
    ///
    /// No `guard_driven` here, and deliberately: the driven refusal
    /// protects a computed slot from being overwritten with a NUMBER,
    /// and this op writes no number. What a driven slot refuses is the
    /// narrower `SlotUnitFault::NotALiteral` the panel model raises —
    /// an expression has no authored notation to change.
    fn set_slot_unit(
        &mut self,
        node: RecipeNodeId,
        slot: SlotId,
        unit: Option<UnitDef>,
    ) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        match props::slot_unit_edit(self.committed_doc(), node, slot, unit) {
            Ok(edit) => self.commit(edit),
            Err(fault) => OpOutcome::refused(Refusal::SlotUnit(fault)),
        }
    }

    fn set_slot_expression(&mut self, node: RecipeNodeId, slot: SlotId, text: &str) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        // `parse_expr` needs the document's declared dimensions so a
        // parameter reference records the dimension `apply` will
        // re-check it against.
        let dims: std::collections::BTreeMap<ParamName, Dimension> = self
            .committed_doc()
            .params()
            .iter()
            .map(|(name, param)| (name.clone(), param.dim()))
            .collect();
        let expr = match parse_expr(text, &dims) {
            Ok(expr) => expr,
            Err(error) => return OpOutcome::refused(Refusal::Parse(Box::new(error))),
        };
        // An expression edit is available on a driven slot AND on a
        // literal one — the refusal is about writing a number over a
        // computation, not about the slot being off limits.
        let edit = if slot.is_structural() {
            DocEdit::SetStructuralParam { node, slot, expr }
        } else {
            DocEdit::SetParam { node, slot, expr }
        };
        self.commit(edit)
    }

    fn set_param(&mut self, name: &ParamName, value: SlotValue) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if !self.committed_doc().params().contains_key(name) {
            return OpOutcome::refused(Refusal::NoSuchParam(name.clone()));
        }
        self.commit(props::param_edit(name.clone(), value))
    }

    /// The create door: refuse an already-declared name typed, commit
    /// the edit for a new one. See [`SessionOp::CreateParam`] for why
    /// this door narrows the edit's create-or-replace semantics.
    fn create_param(&mut self, name: ParamName, value: DocParam) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if let Some(existing) = self.committed_doc().params().get(&name) {
            return OpOutcome::refused(Refusal::ParamExists {
                dimension: existing.dim(),
                name,
            });
        }
        self.commit(DocEdit::SetDocParam { name, value })
    }

    fn begin_gesture(&mut self, node: RecipeNodeId, slot: SlotId) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if let Err(refusal) = self.guard_driven(node, slot) {
            return OpOutcome::refused(refusal);
        }
        let unit = props::slot_unit(self.committed_doc(), node, slot);
        self.start(GestureTarget::Slot { node, slot, unit })
    }

    fn begin_param_gesture(&mut self, name: &ParamName) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let Some(dimension) = self.committed_doc().params().get(name).map(|p| p.dim()) else {
            return OpOutcome::refused(Refusal::NoSuchParam(name.clone()));
        };
        self.start(GestureTarget::Param {
            name: name.clone(),
            dimension,
        })
    }

    /// Open a gesture on an already-validated target.
    fn start(&mut self, target: GestureTarget) -> OpOutcome {
        self.gesture = Some(Gesture {
            target,
            base: self.history.doc().clone(),
            value: None,
        });
        OpOutcome::default()
    }

    fn preview_gesture(&mut self, value: f64) -> OpOutcome {
        let Some(gesture) = self.gesture.as_mut() else {
            return OpOutcome::refused(Refusal::NoGesture);
        };
        let slot_value = gesture.target.value_of(value);
        let edit = match gesture.target.edit(slot_value) {
            Ok(edit) => edit,
            Err(error) => return OpOutcome::refused(Refusal::Dimension(error)),
        };
        // Applied to the gesture's BASE, so previews replace one
        // another instead of composing, and the history never sees any
        // of them.
        match apply(&gesture.base, &edit, self.tol) {
            Ok(applied) => {
                gesture.value = Some(slot_value);
                self.scratch = Some(applied.doc);
                self.request_eval();
                OpOutcome {
                    previewed: vec![edit],
                    ..OpOutcome::default()
                }
            }
            Err(error) => OpOutcome::refused(Refusal::Edit(Box::new(error))),
        }
    }

    fn commit_gesture(&mut self) -> OpOutcome {
        let Some(gesture) = self.gesture.take() else {
            return OpOutcome::refused(Refusal::NoGesture);
        };
        let previewed = self.scratch.take().is_some();
        // A gesture that never moved commits nothing: one undo step
        // per gesture that CHANGED something, none for a click that
        // happened to land on a slider. It also asks for NOTHING —
        // dropping a scratch that was never set leaves the shown
        // document exactly as it was, and a request for a picture we
        // already have spends a generation and flickers the indicator
        // to say so.
        let Some(value) = gesture.value else {
            if previewed {
                self.request_eval();
            }
            return OpOutcome::default();
        };
        match gesture.target.edit(value) {
            Ok(edit) => self.commit(edit),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// Undo (`toward_root`) or redo.
    fn step(&mut self, toward_root: bool) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let moved = if toward_root {
            self.history.undo()
        } else {
            self.history.redo()
        };
        if moved.is_none() {
            return OpOutcome::refused(Refusal::NothingToDo);
        }
        // The document moved, so the display state's derived facts
        // (which instances exist; which are mate-constrained) may have
        // too — an undo past a mate's insertion does NOT resurrect a
        // discarded probe (the value is gone, not parked), but a redo
        // over one discards the probe it constrains.
        let superseded = self.display.prune(self.history.doc());
        self.request_eval();
        OpOutcome {
            superseded,
            ..OpOutcome::default()
        }
    }

    /// Refused mid-gesture, the same policy as [`SessionOp::NewDocument`]:
    /// both replace the document a drag is previewing against, and a
    /// gesture silently dissolved under the pointer is the kind of
    /// half-acted state the gesture guard exists to refuse.
    fn open(&mut self, path: &Path) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        match docio::open(path, self.tol) {
            Ok(history) => {
                // The directory rule: the resolver is the opened
                // file's own directory, consulted lazily at each
                // resolution (DirResolver's docs carry the posture).
                self.resolver = Some(Arc::new(DirResolver::new(session_dir(path))));
                self.history = history;
                self.selection = Selection::None;
                self.hover = None;
                // The landed run answered the PREVIOUS document. Left
                // in place it would render the old model's tree and
                // resolve the new document's names against it until
                // the first run lands.
                self.landed = None;
                self.landed_doc = None;
                self.landed_fault = None;
                self.landed_at_rest = None;
                self.landed_checks = None;
                self.landed_generation = None;
                self.scratch = None;
                self.path = Some(path.to_path_buf());
                // G3: hide and free-move are display state of a
                // SESSION over a document, never of the document — a
                // fresh open starts with none, which is also what
                // makes "save, reopen, layer-3 state gone" a property
                // of the structure.
                self.display.clear();
                self.request_eval();
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(Refusal::Io(Box::new(error))),
        }
    }

    fn save(&mut self, path: &Path) -> OpOutcome {
        match docio::save_path(path, &self.history, self.tol) {
            Ok(()) => {
                self.path = Some(path.to_path_buf());
                // Save-as moves the backing file, and the directory
                // rule follows the file: rebind the resolver to the
                // new parent and re-evaluate, since references may
                // resolve differently there.
                let dir = session_dir(path);
                if self.resolver.as_deref().map(DirResolver::dir) != Some(dir.as_path()) {
                    self.resolver = Some(Arc::new(DirResolver::new(dir)));
                    self.request_eval();
                }
                OpOutcome::default()
            }
            Err(error) => OpOutcome::refused(Refusal::Io(Box::new(error))),
        }
    }

    /// Replace the session with a fresh empty document — the
    /// [`SessionOp::NewDocument`] semantics (see that arm's docs for
    /// the identity ruling and what is cleared).
    ///
    /// The same clearing walk as [`DocSession::open`], with the two
    /// file-shaped fields going the other way: no path and no
    /// resolver, because nothing backs this document until it is
    /// saved.
    fn new_document(&mut self, name: &str) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let name = name.trim();
        if name.is_empty() {
            return OpOutcome::refused(Refusal::EmptyName);
        }
        self.history = History::new(Doc::empty_derived(name, self.tol));
        self.selection = Selection::None;
        self.hover = None;
        self.landed = None;
        self.landed_doc = None;
        self.landed_fault = None;
        self.landed_at_rest = None;
        self.landed_checks = None;
        self.landed_generation = None;
        self.scratch = None;
        self.path = None;
        self.resolver = None;
        self.display.clear();
        self.request_eval();
        OpOutcome::default()
    }

    /// Insert one datum node with literal slots
    /// ([`SessionOp::AddDatum`]).
    fn add_datum(&mut self, datum: DatumSpec) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        match datum_node(datum) {
            Ok(node) => self.commit(DocEdit::InsertNode { node }),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// Insert one profile node from template loops
    /// ([`SessionOp::AddProfile`]).
    ///
    /// No loop-count or nesting judgment here: an empty list, a
    /// degenerate loop and a non-nested pair all go to `commit`, where
    /// the edit door's authoring-time check refuses them typed in the
    /// profile layer's own words — the one rule authored and
    /// hand-written programs share.
    fn add_profile(&mut self, plane: SketchPlane<f64>, loops: &[ProfileShape]) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        let mut programs = Vec::with_capacity(loops.len());
        for shape in loops {
            match loop_program(*shape) {
                Ok(program) => programs.push(program),
                Err(error) => return OpOutcome::refused(Refusal::Dimension(error)),
            }
        }
        self.commit(DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane,
                loops: programs,
            }),
        })
    }

    /// Insert one extrude of an existing profile
    /// ([`SessionOp::AddExtrude`]).
    fn add_extrude(&mut self, profile: RecipeNodeId, distance: f64) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if let Err(refusal) = self.require_kind(profile, NodeKindWanted::Profile) {
            return OpOutcome::refused(refusal);
        }
        match Expr::literal(distance, Dimension::Length) {
            Ok(distance) => self.commit(DocEdit::InsertNode {
                node: Node::Extrude { profile, distance },
            }),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// Insert one revolve of an existing profile about an existing
    /// axis datum ([`SessionOp::AddRevolve`]).
    fn add_revolve(&mut self, profile: RecipeNodeId, axis: RecipeNodeId, angle: f64) -> OpOutcome {
        if self.gesture.is_some() {
            return OpOutcome::refused(Refusal::GestureInFlight);
        }
        if let Err(refusal) = self.require_kind(profile, NodeKindWanted::Profile) {
            return OpOutcome::refused(refusal);
        }
        if let Err(refusal) = self.require_kind(axis, NodeKindWanted::Axis) {
            return OpOutcome::refused(refusal);
        }
        match Expr::literal(angle, Dimension::Angle) {
            Ok(angle) => self.commit(DocEdit::InsertNode {
                node: Node::Revolve {
                    profile,
                    axis,
                    angle,
                },
            }),
            Err(error) => OpOutcome::refused(Refusal::Dimension(error)),
        }
    }

    /// The node-kind gate every creation seat shares: the named node
    /// must be the wanted kind in the committed document — absent and
    /// wrong-kind refuse the same arm, because both mean "there is
    /// nothing of that kind there to consume".
    fn require_kind(&self, node: RecipeNodeId, wanted: NodeKindWanted) -> Result<(), Refusal> {
        let held = self.committed_doc().node(node);
        let ok = match wanted {
            NodeKindWanted::Profile => matches!(held, Some(Node::Profile(_))),
            NodeKindWanted::Axis => matches!(held, Some(Node::Datum(Datum::Axis { .. }))),
        };
        if ok {
            Ok(())
        } else {
            Err(Refusal::WrongNodeKind { node, wanted })
        }
    }

    /// **The one door an edit enters the document through**: apply,
    /// record, re-evaluate — and reconcile the display state, which is
    /// where a free-move probe is superseded by the mate that
    /// constrains its instance (the prune DISCARDS the value; see
    /// `display`'s module docs for why discard and not zero).
    fn commit(&mut self, edit: DocEdit<ProfileProgram>) -> OpOutcome {
        self.commit_action(vec![edit])
    }

    /// **Delete a feature and everything downstream of it**, as one
    /// action and therefore one undo.
    ///
    /// A recipe node that anything consumes cannot leave the document
    /// alone — `DocEdit::DeleteNode` refuses a delete that would
    /// dangle a live reference, and in a chain-shaped model (a boolean
    /// per feature) that is every node but the last. The whole
    /// dependent cone therefore goes, consumers first, which is the
    /// order [`cascade_delete_order`] hands back and the order the
    /// door accepts.
    ///
    /// **The cone, not the chain's remainder**: nodes whose only tie
    /// to the target is that they fed it survive as roots of their own,
    /// so deleting one pip's boolean out of a die leaves that pip's
    /// body in the document, unconsumed. Reconnecting a deleted node's
    /// consumers to its input instead — splice, the CAD-conventional
    /// delete — needs an edit that rewires a live node's inputs, which
    /// this vocabulary does not have; it is open as issue #1324.
    ///
    /// An id the document does not hold takes the single-edit path so
    /// the typed refusal comes from the door rather than from here.
    fn delete_node(&mut self, node: RecipeNodeId) -> OpOutcome {
        let doomed = cascade_delete_order(self.committed_doc(), node);
        if doomed.is_empty() {
            return self.commit(DocEdit::DeleteNode { id: node });
        }
        self.commit_action(
            doomed
                .into_iter()
                .map(|id| DocEdit::DeleteNode { id })
                .collect(),
        )
    }

    /// The same door for an action that takes SEVERAL edits: apply
    /// them in order, and record the whole run as one history state,
    /// so one user action is one undo.
    ///
    /// **All or nothing**: each edit is applied to the value the last
    /// one produced and nothing is recorded until every one has
    /// succeeded, so a refusal anywhere leaves the session on the
    /// document it started from. That is purity doing the work — no
    /// rollback exists to be got wrong.
    fn commit_action(&mut self, edits: Vec<DocEdit<ProfileProgram>>) -> OpOutcome {
        // Threaded rather than cloned up front: the first `apply`
        // reads the history's value in place, and each later one reads
        // its predecessor's output, so a group of one costs exactly
        // what a single commit always cost.
        assert!(!edits.is_empty(), "an action commits at least one edit");
        let mut produced: Option<Doc<ProfileProgram>> = None;
        for edit in &edits {
            let attempt = {
                let base = produced.as_ref().unwrap_or_else(|| self.history.doc());
                apply(base, edit, self.tol)
            };
            match attempt {
                Ok(applied) => produced = Some(applied.doc),
                Err(error) => return OpOutcome::refused(Refusal::Edit(Box::new(error))),
            }
        }
        let Some(doc) = produced else {
            unreachable!("the loop applied at least one edit and kept its output")
        };
        self.history.commit_group(edits.clone(), doc);
        let superseded = self.display.prune(self.history.doc());
        self.request_eval();
        OpOutcome {
            committed: edits,
            superseded,
            ..OpOutcome::default()
        }
    }

    /// **The one submit**: mint the next generation and hand the shown
    /// document to the seam.
    fn request_eval(&mut self) {
        // **Every route that changes the shown document passes here**,
        // which is why the range probe is discarded here and nowhere
        // else: a commit, a gesture preview, an undo, an open. The one
        // caller that does not change the document is `Reevaluate`, and
        // discarding for it too is the conservative direction — a range
        // recomputed on request costs a button press, a stale one costs
        // a wrong decision.
        self.bounds = None;
        self.generation = self.generation.next();
        // ONE clone of the shown document per request: into the
        // `Arc` the session keeps, then out of it into the request the
        // seam owns. The second is the seam's vocabulary (`EvalRequest`
        // takes a value so a worker owns its copy) and is not a
        // retained copy — the session keeps exactly one.
        self.requested_doc = Arc::new(self.doc().clone());
        self.eval.submit(EvalRequest {
            generation: self.generation,
            doc: self.requested_doc.as_ref().clone(),
            tol: self.tol,
            resolver: self
                .resolver
                .as_ref()
                .map(|ws| Arc::clone(ws) as Arc<dyn PartResolver>),
        });
    }
}

/// Lower one datum spec to its node, minting the literal slots —
/// origins and positions Length, normals and directions Scalar (the
/// [`Datum`] slot dimensions).
///
/// # Errors
///
/// A non-finite component (the literal door's refusal).
fn datum_node(spec: DatumSpec) -> Result<Node<ProfileProgram>, DimensionError> {
    let lengths = |v: [f64; 3]| -> Result<[Expr; 3], DimensionError> {
        Ok([
            Expr::literal(v[0], Dimension::Length)?,
            Expr::literal(v[1], Dimension::Length)?,
            Expr::literal(v[2], Dimension::Length)?,
        ])
    };
    let scalars = |v: [f64; 3]| -> Result<[Expr; 3], DimensionError> {
        Ok([
            Expr::literal(v[0], Dimension::Scalar)?,
            Expr::literal(v[1], Dimension::Scalar)?,
            Expr::literal(v[2], Dimension::Scalar)?,
        ])
    };
    Ok(Node::Datum(match spec {
        DatumSpec::Plane { origin, normal } => Datum::Plane {
            origin: lengths(origin)?,
            normal: scalars(normal)?,
        },
        DatumSpec::Axis { origin, direction } => Datum::Axis {
            origin: lengths(origin)?,
            direction: scalars(direction)?,
        },
        DatumSpec::Point { position } => Datum::Point {
            position: lengths(position)?,
        },
    }))
}

/// Lower one template shape to its loop program, through the
/// program's own literal constructors.
///
/// # Errors
///
/// A non-finite field (the constructors' refusal). Degeneracy — a
/// zero radius, a zero width — is NOT judged here: the edit door's
/// authoring-time check replays the program and refuses it typed,
/// which is one rule for authored and hand-written programs alike.
fn loop_program(shape: ProfileShape) -> Result<LoopProgram, DimensionError> {
    match shape {
        ProfileShape::Circle { centre, radius } => {
            LoopProgram::circle(centre[0], centre[1], radius)
        }
        ProfileShape::Rectangle { width, height } => {
            let (hw, hh) = (width / 2.0, height / 2.0);
            // Counter-clockwise from the lower-left corner — the same
            // winding every literal outer loop in this workspace uses.
            LoopProgram::polygon([(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)])
        }
    }
}

/// One evaluation of one document, outside the seam.
///
/// **The seam is for the PICTURE**; this is for a question asked about
/// a document nobody is going to look at (a range probe's candidate).
/// Routing it through the seam would cancel the run the viewport is
/// waiting for — the seam's ruled cancel-and-restart policy — which is
/// exactly the wrong trade for a query the user asked for BESIDE the
/// picture rather than instead of it.
///
/// A fresh `CancelToken` per call, never set: these runs are bounded by
/// the probe's sample cap, and nothing exists to cancel them from.
fn evaluate_with(
    doc: &Doc<ProfileProgram>,
    prior: Option<&Evaluation<f64>>,
    resolver: &Option<Arc<dyn PartResolver>>,
    tol: Tol,
) -> Evaluation<f64> {
    evaluate(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions {
            resolver: resolver.clone(),
            ..EvalOptions::default()
        },
        tol,
    )
}

/// The at-rest verdict for one landed pair, taken only for
/// assembly-shaped documents (see [`AtRestBadge`]). The gate's own
/// vocabulary either way: a certification with its minted count, or
/// the typed refusal rendered by its own `Display`.
fn at_rest_of(doc: &Doc<ProfileProgram>, eval: &Evaluation<f64>, tol: Tol) -> Option<AtRestBadge> {
    let assembly_shaped = doc
        .order()
        .iter()
        .any(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })));
    if !assembly_shaped {
        return None;
    }
    Some(match assemble(doc, eval, tol) {
        Ok(assembly) => AtRestBadge::Certified {
            minted: assembly.minted.len(),
        },
        Err(refusal) => AtRestBadge::Refused {
            message: refusal.to_string(),
        },
    })
}

/// The directory a document at `path` resolves against — its parent,
/// with a bare filename reading as the current directory.
fn session_dir(path: &Path) -> PathBuf {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}

impl std::fmt::Debug for DocSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocSession")
            .field("generation", &self.generation)
            .field("landed_generation", &self.landed_generation)
            .field("selection", &self.selection)
            .field("hover", &self.hover)
            .field("states", &self.history.len())
            .field("gesture", &self.gesture.is_some())
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}
