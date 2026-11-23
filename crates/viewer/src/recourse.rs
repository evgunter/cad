//! **What a failure offers** — the one map from a failure to the acts
//! its recourse names that this viewer can take (DESIGN
//! `[one-way.places]`, "A refusal's recourse"): [`offers`].
//!
//! A failure's acts come in three kinds:
//! - an [`Offer`] drawn beside it: a [`Commit`] (one click, one undo, its
//!   op filled from what the failure carries as data), a [`Lead`] (opens
//!   the form that act is authored in, pre-filled with what the failure
//!   determines, the rest left to the person), or an act it types that
//!   no door of this viewer takes, said in plain words with no control
//!   ([`Unoffered`]), covering only its own clause;
//! - a [`Heard`] lead, taken as the refusal that makes it is heard: the
//!   form opens at once, so it is never in a drawn list.
//!
//! **Staleness.** An offer is drawn when what it was read from
//! describes what its op acts on. A document-editing offer is read off
//! a landed run only through a [`Ran`], which only [`Current`] hands
//! out, and only while that run is the committed document's
//! ([`crate::session::DocSession::current`]); words answer the failure
//! and are read off any run ([`node_words`]); a refusal's acts answer
//! the batch that was refused; re-evaluating is always current. No
//! offer carries a generation to its door: each door keeps its own
//! check, and an offer taken in a frame an earlier op of which moved the
//! document is dropped (`ViewerApp::perform_batch`).
//!
//! Every match below is exhaustive, so a failure kind added to the
//! kernel or the session fails to compile here until it is answered; a
//! catch-all arm is a lint error in this module, so none can be added.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

#![deny(clippy::wildcard_enum_match_arm)]

use pncad::document::{
    CarriedAssertion, CheckEvidence, CheckFinding, LeverRefusal, MateFault, NodeErrorKind,
    OpposedPair, ParseError, PartFault, ProductError, ProductErrorKind, RecipeNodeId, ResolveFault,
    Speaker, VarName,
};
use pncad::workspace::WorkspaceError;

use crate::parts::PartFiles;
use crate::session::{Current, Refusal, SaveChoice, SessionOp};

/// **One act drawn beside a failure.**
#[derive(Debug, Clone, PartialEq)]
pub enum Offer {
    /// One click, one undo.
    Commit(Box<Commit>),
    /// Opens the act's form, pre-filled.
    Lead(Lead),
    /// An act the failure names that this viewer has no door for.
    Unoffered(Unoffered),
}

/// **An act taken in one click**: the op, and the label of its control.
#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    /// The control's label: the act, as the recourse names it.
    pub label: String,
    /// What the click performs.
    pub op: SessionOp,
}

/// **A form a drawn offer opens**, with what the failure determines
/// filled in.
#[derive(Debug, Clone, PartialEq)]
pub enum Lead {
    /// The assertion tool, a `Gap` bounded from above, its two seats
    /// the faces an interference's quieting `Gap` is written over
    /// ([`crate::combine::AssertionTool::led`]). How far below zero is
    /// the person's.
    AssertGap {
        /// The two faces, as the kernel minted their references.
        pair: Box<OpposedPair>,
        /// The control's label.
        label: String,
    },
    /// The placement editor on a placement whose bundle the mate solve
    /// refused, overconstrained or leaving a freedom unwritten: the
    /// edit its recourse names (strike a constraint, write a freedom)
    /// is the editor's.
    EditPlacement {
        /// The placement.
        placement: RecipeNodeId,
        /// The control's label.
        label: String,
    },
}

/// **A form a refusal opens as it is heard**, filled with what the
/// refusal determines.
#[derive(Debug, Clone, PartialEq)]
pub enum Heard {
    /// The add-variable form, its name the one a refused expression
    /// read; the variable's dimension is the person's, since the
    /// expression's context does not determine it.
    CreateVariable {
        /// The name to create.
        name: VarName,
    },
    /// The placement editor on a copy whose target change a door held
    /// over the copy's own constraints, its target set to the door's,
    /// so the rows reading the old space are marked and the strike
    /// offered.
    HoldTarget {
        /// The copy's placement.
        placement: RecipeNodeId,
        /// The target the door was asked for.
        target: crate::session::PlaceTarget,
    },
}

/// **An act a failure types that this viewer has no door for**, said
/// in plain words instead of drawn as a control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unoffered {
    /// The tolerance: no door of this viewer edits it, and a session's
    /// ε is fixed when it is opened.
    Tolerance,
}

impl Unoffered {
    /// The words, covering this act alone.
    pub fn words(self) -> &'static str {
        match self {
            Self::Tolerance => {
                "this viewer cannot change the tolerance: a session's tolerance is fixed when \
                 the document is opened"
            }
        }
    }
}

impl Offer {
    /// The commit of `op`, its control labelled `label`.
    pub fn commit(label: impl Into<String>, op: SessionOp) -> Self {
        Self::Commit(Box::new(Commit {
            label: label.into(),
            op,
        }))
    }

    /// The control's label, `None` for words.
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::Commit(commit) => Some(&commit.label),
            Self::Lead(Lead::AssertGap { label, .. } | Lead::EditPlacement { label, .. }) => {
                Some(label)
            }
            Self::Unoffered(_) => None,
        }
    }
}

/// **What a failure offers**: the acts drawn beside it, and the leads
/// taken as it is heard.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Offers {
    /// Drawn beside the failure.
    pub drawn: Vec<Offer>,
    /// Taken as the failure is heard.
    pub heard: Vec<Heard>,
}

impl Offers {
    fn drawn(drawn: Vec<Offer>) -> Self {
        Self {
            drawn,
            heard: Vec::new(),
        }
    }
}

/// **A failure the viewer can be asked what it offers.**
#[derive(Clone, Copy)]
pub enum Failure<'a> {
    /// A failure the landed run reports, while it is the committed
    /// document's run.
    Run(Ran<'a>),
    /// A batch the session refused.
    Refused(&'a Refusal),
    /// The run was cancelled and the picture shows an older document.
    Canceled,
}

/// **A failure a current run reports**, as only [`Current`] hands one
/// out ([`Current::finding`], [`Current::gather_refusal`], [`Current::node`]):
/// what it was read from is the run whose document its offers act on.
#[derive(Clone, Copy)]
pub struct Ran<'a> {
    current: Current<'a>,
    what: What<'a>,
}

/// What a [`Ran`] is.
#[derive(Clone, Copy)]
enum What<'a> {
    Finding(&'a CheckFinding),
    Product(&'a ProductError),
    Node(&'a NodeErrorKind),
}

impl<'a> Ran<'a> {
    /// `finding`, of `current`'s report.
    pub(crate) fn finding(current: Current<'a>, finding: &'a CheckFinding) -> Self {
        Self {
            current,
            what: What::Finding(finding),
        }
    }

    /// `fault`, `current`'s gather refusal.
    pub(crate) fn gather_refusal(current: Current<'a>, fault: &'a ProductError) -> Self {
        Self {
            current,
            what: What::Product(fault),
        }
    }

    /// A node's own failure `kind`, in `current`'s evaluation.
    pub(crate) fn node(current: Current<'a>, kind: &'a NodeErrorKind) -> Self {
        Self {
            current,
            what: What::Node(kind),
        }
    }
}

/// The label of the control that adds a body to the product.
pub const ADD_TO_THE_PRODUCT: &str = "Add to the product";

/// **The name the pin's recourse gives the accept**
/// (`pncad::workspace::PIN_MISMATCH_RECOURSE`, "record the \"accept
/// updated version\" edit"), which the control's label begins with, so
/// the failure and the control name one act.
pub const ACCEPT_UPDATED_VERSION: &str = "Accept updated version";

/// The label of the control that re-evaluates after a cancel.
pub const REEVALUATE: &str = "Re-evaluate";

/// The label of the control that opens a refused bundle's placement
/// editor.
pub const EDIT_THE_PLACEMENT: &str = "Edit the placement";

/// **The offers `failure` makes**, in the order its recourse names them.
pub fn offers(failure: Failure<'_>) -> Offers {
    match failure {
        Failure::Run(Ran { current, what }) => Offers::drawn(match what {
            What::Finding(finding) => finding_offers(current, finding),
            What::Product(fault) => product_offers(current, fault),
            What::Node(kind) => node_offers(kind, current.files()),
        }),
        Failure::Refused(refusal) => refusal_offers(refusal),
        // A cancel stops the run of the document as it stands, so the
        // same document re-evaluates whatever it is now.
        Failure::Canceled => Offers::drawn(vec![Offer::commit(REEVALUATE, SessionOp::Reevaluate)]),
    }
}

/// A check finding's offers: the assertion it carries
/// ([`CheckFinding::carried_assertion`], the one source of whether it
/// offers one), and an unused body's product door.
fn finding_offers(current: Current<'_>, finding: &CheckFinding) -> Vec<Offer> {
    let doc = current.doc();
    let by = Speaker::of(doc);
    // The assertion the kernel minted by the rule that quiets the row,
    // authored as it is: never seats built from the finding's cell
    // names, which are keyed by the `⊔` that gathered them.
    let assertion = finding.carried_assertion().map(|carried| match carried {
        CarriedAssertion::Claim(quieting) => Offer::commit(
            format!("Assert {}", quieting.said(by)),
            SessionOp::AssertQuieting {
                quieting: quieting.clone(),
            },
        ),
        CarriedAssertion::Pair(pair) => Offer::Lead(Lead::AssertGap {
            label: format!("Assert a negative Gap between {}…", pair.said(by)),
            pair: Box::new(pair.clone()),
        }),
    });
    // The body is offered to the product where the finding's node draws
    // one ([`crate::world::body_read`], the door's own test).
    let product = match &finding.evidence {
        CheckEvidence::Unused { .. } => crate::frame::finding_node(finding, doc)
            .filter(|&node| crate::world::body_read(doc, node, None).is_some())
            .map(|node| {
                Offer::commit(
                    ADD_TO_THE_PRODUCT,
                    SessionOp::AddToProduct { node, body: None },
                )
            }),
        // Each names an edit no one door makes (the geometry, the
        // expectation, a kernel capability), is answered in a part's
        // document, or is the assertion above.
        CheckEvidence::Connectedness { .. }
        | CheckEvidence::Escalated { .. }
        | CheckEvidence::Unsupported { .. }
        | CheckEvidence::NotAtRest { .. }
        | CheckEvidence::StaleExpectation { .. }
        | CheckEvidence::UnprovenCoincidence { .. }
        | CheckEvidence::Interference { .. }
        | CheckEvidence::CarriedInterference { .. }
        | CheckEvidence::CarriedCoincidence { .. }
        | CheckEvidence::AtRest { .. } => None,
    };
    assertion.into_iter().chain(product).collect()
}

/// **A product fault's offers** (D10 `[operations]`): for a slot that
/// reads nothing or a body that is gone, **Add to the product** on each
/// body the picture draws outside the product; for a slot reading a
/// copy outside the world, the same door on each such copy it can move
/// back ([`crate::world::movable_into_world`]).
fn product_offers(current: Current<'_>, fault: &ProductError) -> Vec<Offer> {
    let doc = current.doc();
    let by = Speaker::of(doc);
    let add =
        |label: String, node| Offer::commit(label, SessionOp::AddToProduct { node, body: None });
    match fault.kind() {
        ProductErrorKind::EmptyProduct | ProductErrorKind::StrandedProduct => {
            let picture = crate::world::Picture::of(doc);
            picture
                .nodes()
                .into_iter()
                .filter(|&node| !picture.product.iter().any(|piece| piece.node == node))
                .filter(|&node| crate::world::body_read(doc, node, None).is_some())
                .map(|node| {
                    add(
                        format!(
                            "Add {} to the product",
                            pncad::document::sentence!(by.node(node))
                        ),
                        node,
                    )
                })
                .collect()
        }
        ProductErrorKind::ProductOutsideWorld => crate::world::product_reads(doc)
            .into_iter()
            .filter_map(|var| doc.operation_of(var))
            .filter(|&node| crate::world::movable_into_world(doc, node).is_some())
            .map(|node| {
                add(
                    format!(
                        "Move {} into the world",
                        pncad::document::sentence!(by.node(node))
                    ),
                    node,
                )
            })
            .collect(),
        // Each is the kernel's or a piece's to answer, in its own
        // sentence: no door of the product's makes it.
        ProductErrorKind::EvaluationOfAnotherDocument
        | ProductErrorKind::UnknownNode
        | ProductErrorKind::ReadFailed
        | ProductErrorKind::ReadPoisoned
        | ProductErrorKind::Frame
        | ProductErrorKind::Graft
        | ProductErrorKind::ProductInvalid
        | ProductErrorKind::ContactLineage => Vec::new(),
    }
}

/// **What a node's own failure's recourse names**, as acts: one
/// exhaustive match over the failure itself.
enum NodeAct {
    /// Nothing this viewer offers.
    None,
    /// Accept the updated version of the part `doc_ref` pins.
    Accept(pncad::document::DocumentId),
    /// The placement editor on the placement, for its bundle.
    EditPlacement(RecipeNodeId),
    /// An act with no door here.
    Unoffered(Unoffered),
}

/// **The act `kind`'s recourse names.**
fn node_act(kind: &NodeErrorKind) -> NodeAct {
    use NodeErrorKind as K;
    match kind {
        K::Part { doc_ref, fault } => match fault {
            PartFault::Unresolved { fault, .. } => match fault {
                ResolveFault::PinMismatch => NodeAct::Accept(doc_ref.id),
                // The part records another ε: its recourse is the
                // tolerance's edit.
                ResolveFault::EpsilonSeam => NodeAct::Unoffered(Unoffered::Tolerance),
                // The part's file cannot be read here.
                ResolveFault::Unresolved => NodeAct::None,
            },
            // The reference resolved, or was never tried: the pin is not
            // what failed. A part whose product fails on a mismatch of
            // its OWN references is repaired in that part, not here.
            PartFault::NoResolver
            | PartFault::PartProductFailed { .. }
            | PartFault::PartProductPoisoned { .. }
            | PartFault::PartProductFailureUnrecorded { .. }
            | PartFault::PartProduct { .. }
            | PartFault::ReferenceCycle { .. }
            | PartFault::DepthExceeded
            | PartFault::NotEntered => NodeAct::None,
        },
        K::Mate(fault) => match &**fault {
            // Strike a constraint, or write the freedoms left: the
            // placement editor's edits.
            MateFault::Overconstrained { placement, .. }
            | MateFault::Unpinned { placement, .. } => NodeAct::EditPlacement(*placement),
            // The parts are smaller than the tolerance can see: commit a
            // finer one.
            MateFault::Unleverable { refusal, .. } => match &**refusal {
                LeverRefusal::BelowZeroBand { .. } => NodeAct::Unoffered(Unoffered::Tolerance),
                LeverRefusal::Reach { .. }
                | LeverRefusal::Shape { .. }
                | LeverRefusal::OutOfRange { .. } => NodeAct::None,
            },
            MateFault::PosesOfAnotherDocument { .. }
            | MateFault::NotOnTheCopy { .. }
            | MateFault::NotTheDatumsOwn { .. }
            | MateFault::KindsDiffer { .. }
            | MateFault::NoChart { .. }
            | MateFault::Backwards { .. }
            | MateFault::Unoriented { .. }
            | MateFault::SideUnevaluated { .. }
            | MateFault::ReadUnevaluated { .. } => NodeAct::None,
        },
        // The document's recorded ε is not the process's: its repair is
        // replaying it in a process whose ε matches, the tolerance's act.
        K::ToleranceConflict { .. } => NodeAct::Unoffered(Unoffered::Tolerance),
        // Each names the geometry, an input, a kernel capability or
        // another document's node, which no one door here makes.
        K::Expr { .. }
        | K::Profile { .. }
        | K::ProfileReplay { .. }
        | K::ProfileLaneReplay { .. }
        | K::Trace { .. }
        | K::Extrude { .. }
        | K::Revolve { .. }
        | K::Tube { .. }
        | K::Split { .. }
        | K::ContactCarry { .. }
        | K::Blend { .. }
        | K::Boolean { .. }
        | K::Transform { .. }
        | K::Mirror { .. }
        | K::Motion { .. }
        | K::Skin { .. }
        | K::Loft { .. }
        | K::CurvedSolidFrontier { .. }
        | K::MissingInput { .. }
        | K::UnresolvedRead { .. }
        | K::SelectResolve { .. }
        | K::ParamBox { .. }
        | K::Seed { .. }
        | K::RecordRefused
        | K::RecordMispaired { .. }
        | K::WrongOperand { .. }
        | K::EmptyOperand { .. }
        | K::EmptyMember { .. }
        | K::NotFinished { .. }
        | K::UnfinishedLeaf { .. }
        | K::EmptyHalf { .. }
        | K::HalfOfAFamily { .. }
        | K::MemberOutOfRange { .. }
        | K::MemberRefused { .. }
        | K::IndicesUnrelated { .. }
        | K::IndexCount { .. }
        | K::IndexCountOutOfRange { .. }
        | K::NoSuchPiece { .. }
        | K::PieceShape { .. }
        | K::DegenerateDirection { .. }
        | K::NonFiniteDirection { .. }
        | K::UnderflowedDirection { .. }
        | K::Band { .. }
        | K::MissingSlot { .. }
        | K::Escalated { .. }
        | K::MeasureEscalated { .. }
        | K::PlacementAxisEscalated { .. }
        | K::PoseDirectionEscalated { .. }
        | K::PoseRead { .. }
        | K::PoseDegenerate { .. }
        | K::PoseScalar { .. }
        | K::CountOutOfRange { .. }
        | K::UnschedulableCycle
        | K::Naming { .. }
        | K::UnionFoldStep { .. }
        | K::Shell { .. }
        | K::ShellLaneUnsupported { .. }
        | K::WitnessBifurcation { .. }
        | K::SpaceMismatch { .. }
        | K::CrossingUnverified { .. }
        | K::MeasureRefUnreadable { .. }
        | K::MeasureNonFinite { .. }
        | K::MeasureNotParallel { .. }
        | K::MeasureUnsupported { .. }
        | K::PayloadExpr { .. }
        | K::AssertionDimension { .. } => NodeAct::None,
    }
}

/// **A node's own failure's offers**: the accept a part whose pin no
/// longer holds offers, the placement editor a refused bundle's
/// recourse names, and the words for an act no door here takes;
/// `files` names a part by its file.
fn node_offers(kind: &NodeErrorKind, files: &PartFiles) -> Vec<Offer> {
    match node_act(kind) {
        NodeAct::None => Vec::new(),
        NodeAct::Accept(part) => vec![Offer::commit(
            format!(
                "{ACCEPT_UPDATED_VERSION} of {}, at every instance of it",
                files.name(part)
            ),
            SessionOp::AcceptPartVersion { id: part },
        )],
        NodeAct::EditPlacement(placement) => vec![Offer::Lead(Lead::EditPlacement {
            placement,
            label: EDIT_THE_PLACEMENT.to_owned(),
        })],
        NodeAct::Unoffered(act) => vec![Offer::Unoffered(act)],
    }
}

/// **The words a node's own failure says of an act no door here
/// takes**: they answer the failure, not the document's state, so they
/// are read off any run that reports it.
pub fn node_words(kind: &NodeErrorKind) -> Vec<Unoffered> {
    match node_act(kind) {
        NodeAct::Unoffered(act) => vec![act],
        NodeAct::None | NodeAct::Accept(_) | NodeAct::EditPlacement(_) => Vec::new(),
    }
}

/// **A refused batch's offers**: the two saves a save that would give
/// the document a second file in one folder offers, drawn; the variable
/// an expression named and no document declares, and the placement
/// editor a door's held target change opens, heard.
fn refusal_offers(refusal: &Refusal) -> Offers {
    match refusal {
        Refusal::Parse(error) => Offers {
            drawn: Vec::new(),
            heard: parse_heard(error),
        },
        Refusal::Workspace(error) => Offers::drawn(workspace_offers(error)),
        Refusal::TargetHeld {
            placement, target, ..
        } => Offers {
            drawn: Vec::new(),
            heard: vec![Heard::HoldTarget {
                placement: *placement,
                target: target.clone(),
            }],
        },
        Refusal::DrivenByExpression { .. }
        | Refusal::NoSuchSlot { .. }
        | Refusal::NoSuchVariable(_)
        | Refusal::NotOffered(_)
        | Refusal::OfferIsNamed(_)
        | Refusal::ConstantRefused { .. }
        | Refusal::EmptyName
        | Refusal::WrongNodeKind { .. }
        | Refusal::NotABodyRead { .. }
        | Refusal::PickedMember { .. }
        | Refusal::StepClear(_)
        | Refusal::Frame(_)
        | Refusal::Edit(_)
        | Refusal::Dimension(_)
        | Refusal::NoGesture
        | Refusal::GestureInFlight
        | Refusal::WrongGesture
        | Refusal::Io(_)
        | Refusal::NothingToDo { .. }
        | Refusal::Display(_)
        | Refusal::SlotUnit(_)
        | Refusal::Range(_)
        | Refusal::NoDocumentDirectory
        | Refusal::SelfInstance { .. }
        | Refusal::ProfileEditStale { .. }
        | Refusal::BundleStale { .. }
        | Refusal::PlaceOnACopy { .. }
        | Refusal::SeveralSources { .. }
        | Refusal::CopyInAnotherSpace { .. }
        | Refusal::SourceIsNamed { .. }
        | Refusal::Unspliced { .. }
        | Refusal::ValueNotRemoved { .. }
        | Refusal::Draft(_)
        | Refusal::PlacementFails { .. }
        | Refusal::StrandedPick { .. }
        | Refusal::InTheProduct { .. }
        | Refusal::ClaimHeld { .. } => Offers::default(),
    }
}

/// **The variable a parse refusal opens the add-variable form on.** The
/// parse door's unknown-variable refusal is deliberate typo-safety —
/// text naming an undeclared variable never creates one — so the act is
/// a lead, the form with the name filled. The parse error carries the
/// identifier as text the lexer read, so the name constructor admits
/// it; its answer is folded rather than trusted.
fn parse_heard(error: &ParseError) -> Vec<Heard> {
    match error {
        ParseError::UnknownParam { name, .. } => VarName::new(name.as_str())
            .ok()
            .map(|name| Heard::CreateVariable { name })
            .into_iter()
            .collect(),
        ParseError::UnexpectedChar { .. }
        | ParseError::UnexpectedEnd { .. }
        | ParseError::UnexpectedToken { .. }
        | ParseError::TrailingInput { .. }
        | ParseError::MalformedNumber { .. }
        | ParseError::IntegerOverflow { .. }
        | ParseError::RatioPartNotInteger { .. }
        | ParseError::UnknownUnit { .. }
        | ParseError::UnknownFunction { .. }
        | ParseError::WrongArity { .. }
        | ParseError::Dimension { .. } => Vec::new(),
    }
}

/// **A workspace refusal's offers**: a save that would give the
/// document a second file in one folder offers to save over the file
/// that holds it, or to save at the chosen name as a new document.
/// Dismissing is the third answer and emits nothing.
fn workspace_offers(error: &WorkspaceError) -> Vec<Offer> {
    match error {
        WorkspaceError::SaveWouldDuplicateId {
            existing,
            requested,
            ..
        } => {
            let choice = SaveChoice::new(existing.clone(), requested.clone());
            vec![
                Offer::commit(choice.over_label(), choice.over()),
                Offer::commit(choice.fork_label(), choice.fork()),
            ]
        }
        WorkspaceError::Io { .. }
        | WorkspaceError::DuplicateId { .. }
        | WorkspaceError::Header { .. }
        | WorkspaceError::UnknownId { .. }
        | WorkspaceError::Load { .. }
        | WorkspaceError::Pin { .. }
        | WorkspaceError::PinMismatch { .. }
        | WorkspaceError::Save { .. }
        | WorkspaceError::SaveWouldReplaceDocument { .. }
        | WorkspaceError::SaveTargetNotInStore { .. }
        | WorkspaceError::RandomnessUnavailable { .. }
        | WorkspaceError::Update { .. } => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::panic)]

    use pncad::document::{PartFault, ProductError, RecipeNodeId, ResolveFault};
    use pncad::workspace::PIN_MISMATCH_RECOURSE;

    use super::*;

    /// **The accept control and the store's recourse name one act**: the
    /// label begins with the edit's name the recourse quotes. Red if
    /// either is re-worded without the other.
    #[test]
    fn the_accept_control_is_the_edit_the_pin_mismatch_recourse_quotes() {
        let quoted = format!("\"{}\"", ACCEPT_UPDATED_VERSION.to_lowercase());
        assert!(
            PIN_MISMATCH_RECOURSE.contains(&quoted),
            "{quoted} in: {PIN_MISMATCH_RECOURSE}"
        );
    }

    /// **Which arm of [`PartFault`] a census witness stands for** — an
    /// exhaustive match, so a new arm does not compile until it has an
    /// index here and a witness below.
    fn part_fault_arm(fault: &PartFault) -> usize {
        match fault {
            PartFault::NoResolver => 0,
            PartFault::Unresolved { fault, .. } => match fault {
                ResolveFault::PinMismatch => 1,
                ResolveFault::EpsilonSeam => 2,
                ResolveFault::Unresolved => 3,
            },
            PartFault::PartProductFailed { .. } => 4,
            PartFault::PartProductPoisoned { .. } => 5,
            PartFault::PartProductFailureUnrecorded { .. } => 6,
            PartFault::PartProduct { .. } => 7,
            PartFault::ReferenceCycle { .. } => 8,
            PartFault::DepthExceeded => 9,
            PartFault::NotEntered => 10,
        }
    }

    /// **Only a pin that no longer holds offers the accept**, and only
    /// the ε seam the tolerance's words, over one
    /// witness of every [`PartFault`] arm, naming the part by the file
    /// the scan found. The part-product arms carry a NESTED pin mismatch:
    /// that pin is the part's own reference, repaired in the part, so
    /// it offers nothing here. Red if any other arm offers, or the
    /// offer names another part.
    #[test]
    fn only_a_pin_that_no_longer_holds_offers_the_accept() {
        use pncad::document::NodeRefusal;

        use crate::test_support::{PART_FILE, part_refused};
        let unresolved = |fault| PartFault::Unresolved {
            fault,
            message: "the store's own words".to_owned(),
        };
        let (mismatch, _) = part_refused(unresolved(ResolveFault::PinMismatch));
        let NodeErrorKind::Part { doc_ref, .. } = &mismatch else {
            unreachable!("the fixture is a part refusal")
        };
        let nested = || NodeRefusal::from(part_refused(unresolved(ResolveFault::PinMismatch)).0);
        let witnesses = [
            PartFault::NoResolver,
            unresolved(ResolveFault::PinMismatch),
            unresolved(ResolveFault::EpsilonSeam),
            unresolved(ResolveFault::Unresolved),
            PartFault::PartProductFailed {
                held: Default::default(),
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(7)),
                refusal: nested(),
            },
            PartFault::PartProductPoisoned {
                held: Default::default(),
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(8)),
                through: RecipeNodeId::new(0, test_utils::refusal::tagged(7)),
                refusal: nested(),
            },
            PartFault::PartProductFailureUnrecorded {
                held: Default::default(),
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(7)),
            },
            PartFault::PartProduct {
                held: Default::default(),
                refusal: ProductError::EmptyProduct.into(),
            },
            PartFault::ReferenceCycle {
                cycle: vec![*doc_ref],
            },
            PartFault::DepthExceeded,
            PartFault::NotEntered,
        ];
        let mut seen = [false; 11];
        for fault in witnesses {
            let arm = part_fault_arm(&fault);
            assert!(!seen[arm], "two witnesses for arm {arm}: {fault:?}");
            seen[arm] = true;
            let (kind, files) = part_refused(fault);
            let made = node_offers(&kind, &files);
            if arm == part_fault_arm(&unresolved(ResolveFault::PinMismatch)) {
                let [Offer::Commit(commit)] = &made[..] else {
                    panic!("a pin mismatch offers the accept alone: {made:?}");
                };
                assert!(
                    matches!(commit.op, SessionOp::AcceptPartVersion { id } if id == doc_ref.id),
                    "{:?}",
                    commit.op
                );
                assert!(
                    commit.label.starts_with(ACCEPT_UPDATED_VERSION)
                        && commit.label.contains(PART_FILE),
                    "the accept names the act and the part by its file: {}",
                    commit.label
                );
            } else if arm == part_fault_arm(&unresolved(ResolveFault::EpsilonSeam)) {
                assert_eq!(
                    made,
                    [Offer::Unoffered(Unoffered::Tolerance)],
                    "the ε seam's act is the tolerance, which no door here takes"
                );
            } else {
                assert!(
                    made.is_empty(),
                    "only the pin and the ε seam offer: {kind:?}"
                );
            }
        }
        assert!(seen.iter().all(|&s| s), "arms with no witness: {seen:?}");
    }

    /// **What every node failure class offers, pinned per class** — the
    /// kernel's census of one witness per class
    /// (`editor_core::test_support::node_error_witnesses`) run through
    /// [`node_offers`]: the pin mismatch commits the accept, the ε seam
    /// and the tolerance conflict say the tolerance's words, an
    /// overconstrained or unpinned bundle leads to its placement editor,
    /// and every other class offers nothing. A lever below the zero band
    /// is the tolerance too; the census's lever witness (an unread
    /// reach) offers nothing.
    ///
    /// Red if any class offers other than this table says, including a
    /// class added to the kernel and not answered here.
    #[test]
    // The census's last arm is the table's "every other class": a class
    // added to the kernel lands there and is held to offering nothing.
    #[allow(clippy::wildcard_enum_match_arm)]
    fn every_node_failure_class_offers_what_its_recourse_names() {
        use pncad::document::NodeErrorClass as C;
        let files = PartFiles::Unscanned;
        let mut seen = 0;
        for (class, kind) in
            editor_core::test_support::node_error_witnesses(pncad::geom_core::Tol::witness())
        {
            let made = node_offers(&kind, &files);
            match class {
                C::PartPinMismatch => assert!(
                    matches!(&made[..], [Offer::Commit(commit)]
                        if matches!(commit.op, SessionOp::AcceptPartVersion { .. })),
                    "{class:?}: the accept: {made:?}"
                ),
                C::PartEpsilonSeam | C::ToleranceConflict => assert_eq!(
                    made,
                    [Offer::Unoffered(Unoffered::Tolerance)],
                    "{class:?}: the tolerance's words"
                ),
                C::MateOverconstrained | C::MateUnpinned => assert!(
                    matches!(&made[..], [Offer::Lead(Lead::EditPlacement { label, .. })]
                        if label == EDIT_THE_PLACEMENT),
                    "{class:?}: the placement editor: {made:?}"
                ),
                C::MateUnleverable => {
                    assert!(made.is_empty(), "{class:?}: an unread reach: {made:?}");
                    let NodeErrorKind::Mate(fault) = &kind else {
                        panic!("a mate's witness is a mate fault")
                    };
                    let MateFault::Unleverable { mate, .. } = &**fault else {
                        panic!("the lever's witness is unleverable")
                    };
                    let below = NodeErrorKind::Mate(Box::new(MateFault::Unleverable {
                        mate: *mate,
                        refusal: Box::new(LeverRefusal::BelowZeroBand {
                            arm: 1.0e-12,
                            zero: 1.0e-9,
                        }),
                    }));
                    assert_eq!(
                        node_offers(&below, &files),
                        [Offer::Unoffered(Unoffered::Tolerance)],
                        "a lever below the zero band: the tolerance's words"
                    );
                }
                _ => assert!(made.is_empty(), "{class:?} offers nothing: {made:?}"),
            }
            assert_eq!(
                node_words(&kind),
                made.iter()
                    .filter_map(|offer| match offer {
                        Offer::Unoffered(act) => Some(*act),
                        Offer::Commit(_) | Offer::Lead(_) => None,
                    })
                    .collect::<Vec<_>>(),
                "{class:?}: the words are the offers' words"
            );
            seen += 1;
        }
        assert!(seen > 80, "the census is every class: {seen}");
    }

    /// **The parse door's only offer is the unknown name's**: every other
    /// parse error opens nothing. Red if, say, an unknown function were
    /// read as a variable to create.
    #[test]
    fn only_an_unknown_name_opens_the_add_variable_form() {
        use crate::session::Refusal;
        let doc = pncad::document::Doc::<pncad::document::ProfileProgram>::empty_derived(
            "parse",
            pncad::geom_core::Tol::witness(),
        );
        let heard = |text: &str| {
            let refused = crate::session::parse_text(&doc, text)
                .err()
                .unwrap_or_else(|| panic!("{text:?} does not parse"));
            offers(Failure::Refused(&Refusal::from(refused)))
        };
        assert_eq!(
            heard("margin * 2").heard,
            [Heard::CreateVariable {
                name: VarName::from_static("margin")
            }]
        );
        let frob = crate::session::parse_text(&doc, "frob(1)").err();
        assert!(
            matches!(
                frob.as_ref().map(|r| &*r.0),
                Some(ParseError::UnknownFunction { .. })
            ),
            "the premise: an unknown function: {frob:?}"
        );
        for text in ["frob(1)", "1 +", "1 2", "2 ^^ 3", "1 parsec"] {
            assert_eq!(heard(text), Offers::default(), "{text:?} opens nothing");
        }
    }
}
