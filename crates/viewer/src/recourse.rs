//! **What a failure offers** — the one map from a failure to the acts
//! its recourse names that this viewer can take (DESIGN
//! `[one-way.places]`, "A refusal's recourse"): [`offers`].
//!
//! Each offer is one of two kinds, or words:
//! - a [`Commit`], one click and one undo, its op filled from what the
//!   failure carries as data;
//! - a [`Lead`], which opens the form that act is authored in,
//!   pre-filled with what the failure determines and leaving the rest
//!   to the person;
//! - an act the failure types that no door of this viewer takes, said
//!   in plain words with no control ([`Unoffered`]), covering only its
//!   own clause.
//!
//! **Staleness.** An offer is drawn when what it was read from
//! describes what its op acts on. A document-editing offer is read off
//! a landed run only through [`Current`], which the session hands out
//! only while that run is the committed document's
//! ([`crate::session::DocSession::current`]); a refusal's offers answer
//! the batch that was refused, and are read off the refusal wherever
//! it is held; re-evaluating is always current. No offer carries a
//! generation to its door: each door keeps its own check.
//!
//! Every match below is exhaustive, so a failure kind added to the
//! kernel or the session fails to compile here until it is answered; a
//! catch-all arm is a lint error in this module, so none can be added.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

#![deny(clippy::wildcard_enum_match_arm)]

use pncad::document::{
    CheckEvidence, CheckFinding, NodeErrorClass, NodeErrorKind, OpposedPair, ParseError,
    ProductError, ProductErrorKind, Speaker, VarName,
};
use pncad::workspace::WorkspaceError;

use crate::session::{Current, Refusal, SaveChoice, SessionOp};

/// **One act a failure offers.**
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

/// **A form an offer opens**, with what the failure determines filled
/// in.
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
    /// The add-variable form, its name the one a refused expression
    /// read; the variable's dimension is the person's, since the
    /// expression's context does not determine it.
    CreateVariable {
        /// The name to create.
        name: VarName,
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
            Self::Lead(Lead::AssertGap { label, .. }) => Some(label),
            Self::Lead(Lead::CreateVariable { .. }) | Self::Unoffered(_) => None,
        }
    }

    /// **Whether this offer is the assertion a finding carries**, so
    /// the finding is said without it beside the control
    /// ([`CheckFinding::said_beside_its_assertion`]).
    pub fn authors_the_findings_assertion(&self) -> bool {
        match self {
            Self::Commit(commit) => matches!(commit.op, SessionOp::AssertQuieting { .. }),
            Self::Lead(Lead::AssertGap { .. }) => true,
            Self::Lead(Lead::CreateVariable { .. }) | Self::Unoffered(_) => false,
        }
    }
}

/// **A failure the viewer can be asked what it offers.**
#[derive(Clone, Copy)]
pub enum Failure<'a> {
    /// A failure the landed run reports, read off it while it is the
    /// committed document's run.
    Run(Current<'a>, Ran<'a>),
    /// A batch the session refused.
    Refused(&'a Refusal),
    /// The run was cancelled and the picture shows an older document.
    Canceled,
}

/// **What a landed run reports failing.**
#[derive(Clone, Copy)]
pub enum Ran<'a> {
    /// A check's finding, of the run's report.
    Finding(&'a CheckFinding),
    /// The gather's refusal.
    Product(&'a ProductError),
    /// A node's own failure.
    Node(&'a NodeErrorKind),
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

/// **The offers `failure` makes**, in the order its recourse names them.
pub fn offers(failure: Failure<'_>) -> Vec<Offer> {
    match failure {
        Failure::Run(current, Ran::Finding(finding)) => finding_offers(current, finding),
        Failure::Run(current, Ran::Product(fault)) => product_offers(current, fault),
        Failure::Run(current, Ran::Node(kind)) => node_offers(kind, current.files()),
        Failure::Refused(refusal) => refusal_offers(refusal),
        // A cancel stops the run of the document as it stands, so the
        // same document re-evaluates whatever it is now.
        Failure::Canceled => vec![Offer::commit(REEVALUATE.to_owned(), SessionOp::Reevaluate)],
    }
}

/// A check finding's offers. A quiet finding offers nothing: an
/// assertion already says it.
fn finding_offers(current: Current<'_>, finding: &CheckFinding) -> Vec<Offer> {
    if finding.quiet.is_some() {
        return Vec::new();
    }
    let doc = current.doc();
    let by = Speaker::of(doc);
    match &finding.evidence {
        // The assertion the kernel minted by the rule that quiets the
        // row, authored as it is: never seats built from the finding's
        // cell names, which are keyed by the `⊔` that gathered them.
        CheckEvidence::UnprovenCoincidence { quieting, .. } => quieting
            .iter()
            .map(|quieting| {
                Offer::commit(
                    format!("Assert {}", quieting.said(by)),
                    SessionOp::AssertQuieting {
                        quieting: quieting.clone(),
                    },
                )
            })
            .collect(),
        CheckEvidence::Interference { finding } => finding
            .quieting
            .iter()
            .map(|pair| {
                Offer::Lead(Lead::AssertGap {
                    label: format!("Assert a negative Gap between {}…", pair.said(by)),
                    pair: Box::new(pair.clone()),
                })
            })
            .collect(),
        // The body is offered to the product where the finding's node
        // draws one ([`crate::world::body_read`], the door's own test).
        CheckEvidence::Unused { .. } => crate::frame::finding_node(finding, doc)
            .filter(|&node| crate::world::body_read(doc, node, None).is_some())
            .map(|node| {
                Offer::commit(
                    ADD_TO_THE_PRODUCT.to_owned(),
                    SessionOp::AddToProduct { node, body: None },
                )
            })
            .into_iter()
            .collect(),
        // A part's finding is answered in the part's document.
        CheckEvidence::CarriedInterference { .. } | CheckEvidence::CarriedCoincidence { .. } => {
            Vec::new()
        }
        // Each names an edit no one door makes: the geometry, the
        // expectation, or a kernel capability.
        CheckEvidence::Connectedness { .. }
        | CheckEvidence::Escalated { .. }
        | CheckEvidence::Unsupported { .. }
        | CheckEvidence::NotAtRest { .. }
        | CheckEvidence::StaleExpectation { .. }
        | CheckEvidence::AtRest { .. } => Vec::new(),
    }
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

/// **A node's own failure's offers**, by its class: the accept a part
/// whose pin no longer holds offers, and the tolerance, which no door
/// here edits.
fn node_offers(kind: &NodeErrorKind, files: &crate::parts::PartFiles) -> Vec<Offer> {
    use NodeErrorClass as C;
    match kind.class() {
        C::PartPinMismatch => {
            let NodeErrorKind::Part { doc_ref, .. } = kind else {
                unreachable!("a part's pin mismatch is its Part arm")
            };
            vec![Offer::commit(
                format!(
                    "{ACCEPT_UPDATED_VERSION} of {}, at every instance of it",
                    files.name(doc_ref.id)
                ),
                SessionOp::AcceptPartVersion { id: doc_ref.id },
            )]
        }
        // The part records another ε, or the document does: the edit is
        // the tolerance's.
        C::PartEpsilonSeam | C::ToleranceConflict => vec![Offer::Unoffered(Unoffered::Tolerance)],
        // Each names the geometry, an input, a kernel capability or
        // another document's node, which no one door here makes.
        C::Expr
        | C::Profile
        | C::ProfileReplay
        | C::ProfileLaneReplay
        | C::Trace
        | C::Extrude
        | C::Revolve
        | C::Tube
        | C::Split
        | C::ContactCarry
        | C::Fillet
        | C::Chamfer
        | C::Boolean
        | C::Transform
        | C::Mirror
        | C::Motion
        | C::Skin
        | C::Loft
        | C::CurvedSolidFrontier
        | C::MissingInput
        | C::UnresolvedRead
        | C::SelectResolve
        | C::ParamBox
        | C::Seed
        | C::RecordRefused
        | C::RecordMispaired
        | C::WrongOperand
        | C::EmptyOperand
        | C::EmptyMember
        | C::NotFinished
        | C::UnfinishedLeaf
        | C::EmptyHalf
        | C::HalfOfAFamily
        | C::MemberOutOfRange
        | C::MemberRefused
        | C::IndicesUnrelated
        | C::IndexCount
        | C::IndexCountOutOfRange
        | C::NoSuchPiece
        | C::PieceShape
        | C::DegenerateDirection
        | C::NonFiniteDirection
        | C::UnderflowedDirection
        | C::Band
        | C::MissingSlot
        | C::Escalated
        | C::PoseRead
        | C::PoseDegenerate
        | C::PoseScalar
        | C::CountOutOfRange
        | C::UnschedulableCycle
        | C::Naming
        | C::UnionFoldStep
        | C::Shell
        | C::ShellLaneUnsupported
        | C::WitnessBifurcation
        | C::PartNoResolver
        | C::PartUnresolved
        | C::PartProductFailed
        | C::PartProductPoisoned
        | C::PartProductFailureUnrecorded
        | C::PartProduct
        | C::PartReferenceCycle
        | C::PartDepthExceeded
        | C::PartNotEntered
        | C::MatePosesOfAnotherDocument
        | C::MateOverconstrained
        | C::MateUnpinned
        | C::MateNotOnTheCopy
        | C::MateUnleverable
        | C::MateKindsDiffer
        | C::MateSideUnevaluated
        | C::MateNoChart
        | C::MateBackwards
        | C::MateUnoriented
        | C::SpaceMismatch
        | C::CrossingUnverified
        | C::MeasureRefUnreadable
        | C::MeasureNonFinite
        | C::MeasureNotParallel
        | C::MeasureUnsupported
        | C::PayloadExpr
        | C::AssertionDimension => Vec::new(),
    }
}

/// **A refused batch's offers**: the variable an expression named and
/// no document declares, and the two saves a save that would give the
/// document a second file in one folder offers.
fn refusal_offers(refusal: &Refusal) -> Vec<Offer> {
    match refusal {
        Refusal::Parse(error) => parse_offers(error),
        Refusal::Workspace(error) => workspace_offers(error),
        Refusal::DrivenByExpression { .. }
        | Refusal::NoSuchSlot { .. }
        | Refusal::NoSuchVariable(_)
        | Refusal::VariableIsDefined(_)
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
        | Refusal::NoDocumentDirectory
        | Refusal::SelfInstance { .. }
        | Refusal::ProfileEditStale { .. }
        | Refusal::BundleStale { .. }
        | Refusal::PlaceOnACopy { .. }
        | Refusal::SeveralSources { .. }
        | Refusal::TargetHeld { .. }
        | Refusal::CopyInAnotherSpace { .. }
        | Refusal::SourceIsNamed { .. }
        | Refusal::Unspliced { .. }
        | Refusal::ValueNotRemoved { .. }
        | Refusal::Draft(_)
        | Refusal::PlacementFails { .. }
        | Refusal::StrandedPick { .. }
        | Refusal::InTheProduct { .. } => Vec::new(),
    }
}

/// **The variable a parse refusal offers to create.** The parse door's
/// unknown-variable refusal is deliberate typo-safety — text naming an
/// undeclared variable never creates one — so the offer is a lead, the
/// form with the name filled. The parse error carries the identifier as
/// text the lexer read, so the name constructor admits it; its answer
/// is folded rather than trusted.
fn parse_offers(error: &ParseError) -> Vec<Offer> {
    match error {
        ParseError::UnknownParam { name, .. } => VarName::new(name.as_str())
            .ok()
            .map(|name| Offer::Lead(Lead::CreateVariable { name }))
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
}
