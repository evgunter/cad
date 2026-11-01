//! Layer-3 display state: the author's **hide** of a body or a space,
//! and the **free-move** fit probe (GUI-DESIGN G3), each with exactly
//! one home.
//!
//! # What this state is, and what it is not
//!
//! Both facts here are about the PICTURE, never about the model. A
//! hidden body or space stays in the document and in the feature tree;
//! only the drawn scene and the pick index drop it. A free-move
//! transform is the G3 fit probe — a display frame composed over a shown
//! space's drawn bodies so a user can hold a part against another
//! BEFORE authoring the mate — and it involves no solver, enters no
//! history, and is never persisted: `save` writes the document, and a
//! reopened session starts with this state empty. DI5
//! (`crates/editor-core/IDENTITY.md`) states the rule over shown spaces:
//! a free-move moves the picture of one shown space, its release writes
//! nothing to the document, and the frame is keyed by the space — its
//! `Space` variable — and dropped by [`DisplayState::prune`] once that
//! variable stops naming a shown space
//! ([`crate::world::shown_spaces`]). Keeping a position means stating
//! it, as a placement's mates and typed values.
//!
//! # Hides are keyed by body and by space
//!
//! The picture is drawn per drawn body ([`crate::world::drawn_bodies`]).
//! A body's hide names the node building it and drops what draws it:
//! the body, or for a `⊔` its pieces. A space's hide drops every body
//! drawn in the space. Either is admitted on any live body or space,
//! drawn or not ([`body_check`], [`space_hide_check`]): a hide is the
//! state of a toggle the View pane holds for every body and space.
//!
//! # Free-move admits any shown space but the world's
//!
//! Free-move is opened on a node whose body is drawn in a space, and
//! moves that space: read off the recipe ([`Doc::node_space`]), every
//! body drawn in it moves with it, whatever kind of node drew it. A body
//! in the world refuses ([`AdmissionFault::InTheWorld`]): the world is
//! the frame the other spaces move against. The test also asks an
//! evaluation that answers the document whether the body evaluated, and
//! refuses typed while none has landed ([`AdmissionFault::NotEvaluated`])
//! rather than waiting: a body with no value has no picture to move.
//!
//! # Supersession: the frame goes when its space stops being shown
//!
//! When an edit empties a free-moved space of drawn bodies (an operation
//! reads each as a body, or each is re-targeted out of it) or deletes the
//! space, its frame is **discarded** — removed outright, not zeroed and
//! kept — by [`DisplayState::prune`]. Discard rather than zero because
//! the value's LEGALITY is derived from the document and the evaluation
//! that answers it: a frame for a space that no longer exists is
//! unrepresentable state, not a transform that happens to be identity.
//!
//! The session prunes twice for one edit. A frame is asked of the
//! document alone, at the edit: whether its space is still shown. A
//! gesture is asked again at the landing, whose evaluation says whether
//! its body still evaluates, and the session keeps what that landing
//! withdraws for the chrome to say
//! (`DocSession::take_landed_withdrawals`).
//!
//! # A hide the document can no longer hold is DROPPED, and said
//!
//! The same reconciliation drops a hidden entry whose body or space the
//! document no longer holds. That is not a supersession: nothing
//! replaced the user's choice, it stopped being expressible.
//! [`DisplayState::prune`] therefore reports it beside the discarded
//! probes ([`PruneReport`]) rather than undoing a user's choice in
//! silence.
//!
//! # Which history holds a committed free-move: none
//!
//! The G1 preview/commit shape applies — a gesture streams preview
//! frames and lands exactly one committed value, and its three
//! transition rules are [`crate::g1::Slot`]'s, shared with the value
//! drag — but the commit replaces this state's entry and enters NO
//! history: the plan's undo note governs document state only, and no
//! layer-3 history exists in v1. Undo/redo therefore never change what is hidden or probed;
//! they can only DISCARD a probe by emptying its space, placing its
//! bodies into another, or removing it.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::{BTreeMap, BTreeSet};

use pncad::document::{
    Doc, Evaluation, Frame, OperandSlot, ProfileProgram, RecipeNodeId, SpokenNode, SpokenSpace,
    VarId, VarKind,
};

use crate::g1;

/// **Why a body or a space cannot hold display state** — what the admission
/// tests answer, and the only cause a [`Withdrawn`] carries.
///
/// # Each arm names its subject through the spoken node
///
/// The rule over the whole enum, not a property of one arm: an arm
/// says its subject as [`SpokenNode`] says it — always the node's kind
/// as the document holds it, with its label beside it when it has one
/// (`InstantiatePart N`, `Place "shelf" (N)`) — and never writes the
/// subject's kind in its own words. A frame's withdrawal names the
/// node whose probe moved it and its space, said as [`SpokenSpace`]
/// says it ([`AdmissionFault::NoSuchSpace`], [`AdmissionFault::NotShown`]),
/// and a space is said through the node minting it.
/// An id the document does not hold has no kind and reads **"node N"**
/// ([`AdmissionFault::NoSuchNode`]). The census in
/// `crates/viewer/tests/frame_policy.rs` holds the rule for the word
/// "instance"; the other kind words are held by review.
///
/// A caller rendering these must therefore not promise its reader one
/// vocabulary across all of them.
///
/// # No arm's sentence carries [`crate::frame::LIST_SEPARATOR`]
///
/// [`crate::frame::Withdrawal`] joins a withdrawal's causes with that
/// mark and joins them flat, so a cause whose own sentence writes one
/// reads as an item more than it is — the ambiguity-at-two
/// [`crate::frame::NOTICE_SEPARATOR`] answers one level out.
///
/// **What this type carries is the POPULATION, which is the half a
/// signature CAN hold.** "No cause writes the mark" is a claim about
/// strings and no signature carries it; what the join needs first is
/// to know which sentences it is a claim about. The admission tests
/// answer this enum rather than [`DisplayFault`], and [`Withdrawn`]
/// stores what they answer, so the sentences the claim ranges over
/// are these arms' and a new one cannot arrive without an arm here, or
/// a guard on one, each sampled by the census. The
/// claim itself is then a census over a closed set, held by
/// `a_withdrawn_cause_never_carries_the_list_mark` in
/// `crates/viewer/tests/frame_policy.rs`.
///
/// [`DisplayFault::NonRigidFrame`] is the sentence that makes this
/// worth a type: it writes the mark inside one sentence, it is
/// raised by [`DisplayState::preview_free_move`] alone, and it is
/// outside this enum — so the rendering is untouched and the join
/// still cannot meet it.
#[derive(Debug, Clone, PartialEq)]
pub enum AdmissionFault {
    /// The document holds no node with this id at all — it was
    /// deleted, or an undo stepped back past the edit that made it.
    ///
    /// **Spelled apart from [`AdmissionFault::NotABody`]** because
    /// the two are different news to a user holding display state on
    /// the id: a node that builds no body is a mis-aimed operation, and an
    /// absent one is the thing they were looking at being gone. The
    /// reconciliation in [`DisplayState::prune`] reports the fault it
    /// discards on, so the difference is the whole content of the
    /// sentence the status line then shows.
    ///
    /// **It is decided in [`body_check`]**, which every body's
    /// OPERATION runs first and not only the prune:
    /// [`DisplayState::set_body_hidden`] and the free-move admission
    /// ([`free_move_check`], and so [`DisplayState::begin_free_move`]).
    /// At those doors a user who aims a display operation at an id the
    /// document does not hold reads *node N is not in the document*
    /// where they would read *node N builds no body* — the better
    /// sentence, because the id denotes nothing.
    ///
    /// **The properties panel runs the same test and renders neither
    /// arm, which is not a discard.** `PropertiesPane::display_ui`
    /// draws no display section for either refusal. For this arm
    /// the sentence is already on screen directly above it:
    /// `standing_ui` renders [`crate::session::Standing`]'s `Node`
    /// vanished arm from the SAME lookup on the SAME document in the
    /// same frame, and that type's rule is that a vanished reference
    /// is rendered there (its survival is ratified,
    /// `docs/SELECT-DESIGN.md` §4) *while the affordances that need a
    /// live entity switch off* (`Standing`'s own clause, in no design
    /// doc). The section switching off IS the second
    /// clause; a sentence at the affordance would be one fact spelled
    /// twice in one pane.
    NoSuchNode {
        /// The id named, said by its tag (`node <tag>`) and kept so
        /// when spoken again ([`AdmissionFault::respoken`]).
        node: SpokenNode,
    },
    /// The node builds no body (a profile, a measure, a space), so the
    /// picture has nothing of it to hide or move.
    NotABody {
        /// The node named, as the document held it.
        node: SpokenNode,
    },
    /// The variable named is no space of the document — it never was
    /// one, or the node minting it was deleted — so there is no picture
    /// of it to hide.
    NotASpace {
        /// The variable, as the document said it.
        space: Box<SpokenSpace>,
    },
    /// The node's body is in the world's space, the product's.
    /// Free-move moves the picture of another shown space (DI5) and the
    /// world is drawn where the document puts it, so the probe refuses.
    InTheWorld {
        /// The node, as the document held it.
        node: SpokenNode,
    },
    /// No evaluation of the current document has landed, so whether the
    /// node's body evaluated, and has a picture to move, is not known
    /// yet (DI3: an evaluation names the document it answers, and an
    /// older one's values are not this one's).
    NotEvaluated {
        /// The node, as the document held it.
        node: SpokenNode,
    },
    /// The node's body has no picture: it is in no live space, or its
    /// value did not evaluate (it refused, or an input it reads failed),
    /// so there is nothing of it to move.
    NoSpace {
        /// The node, as the document held it.
        node: SpokenNode,
    },
    /// The body a gesture was opened on is now in another space than the
    /// one the gesture moves: an edit re-targeted it. The gesture goes
    /// (DI5); the probe still admits the body, to the space it is in
    /// now. Raised by the prune's tests alone.
    MovedSpace {
        /// The node the gesture was opened on, as the document held it.
        node: SpokenNode,
        /// The space its body is in now, as the document held it.
        into: Box<SpokenSpace>,
    },
    /// The space a frame was drawn for is gone from the document — the
    /// `new_space()` or the construction minting it was deleted, or an
    /// undo stepped back past it. Raised by the prune's tests alone.
    /// Nothing cascades from a deleted space to the placements into it
    /// (D10): they strand.
    NoSuchSpace {
        /// The node whose probe moved the space, as the document held
        /// it.
        node: SpokenNode,
        /// The space, as the document that last held it said it.
        space: Box<SpokenSpace>,
    },
    /// The space a frame was drawn for, or a probe is asked of, is not
    /// shown ([`crate::world::shown_spaces`]): it holds no body the
    /// picture draws — every body in it is read as a body by an
    /// operation, which is where it is drawn.
    NotShown {
        /// The node whose probe moves, or moved, the space, as the
        /// document held it.
        node: SpokenNode,
        /// The space, as the document held it.
        space: Box<SpokenSpace>,
    },
}

impl core::fmt::Display for AdmissionFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoSuchNode { node } => {
                write!(f, "{node} is not in the document")
            }
            Self::NotABody { node } => {
                write!(
                    f,
                    "{node} builds no body, so the picture draws nothing of it"
                )
            }
            Self::NotASpace { space } => {
                write!(f, "{space} is not a space of the document")
            }
            Self::InTheWorld { node } => write!(
                f,
                "{node} is in the world, the product's space, so the free-move probe \
                 refuses. Free-move moves another shown space"
            ),
            Self::NotEvaluated { node } => write!(
                f,
                "{node} has no picture yet: no evaluation of the current document has \
                 landed"
            ),
            Self::NoSpace { node } => write!(
                f,
                "{node} has no picture: it did not evaluate, so there is nothing of it to \
                 move"
            ),
            Self::MovedSpace { node, into } => write!(
                f,
                "{node} is now in {into}, not the space the free-move was drawn for, so \
                 that free-move is gone. A free-move of {node} moves the space it is in now"
            ),
            Self::NoSuchSpace { node, space } => write!(
                f,
                "{node} was free-moved in {space}, which is not in the document, so that \
                 free-move is gone"
            ),
            Self::NotShown { node, space } => write!(
                f,
                "{node}'s free-move moves {space}, which is not shown — every body in it is \
                 read by an operation and drawn there — so there is no picture of it to move"
            ),
        }
    }
}

impl core::error::Error for AdmissionFault {}

impl AdmissionFault {
    /// This fault with its nodes spoken from `doc`, a later version of
    /// the document it was raised in
    /// ([`crate::session::Refusal::respoken`]).
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        let again = |node: SpokenNode| node.respoken(doc);
        match self {
            // Its sentence is that the node or the space is not there:
            // said from a version that holds it again, it would
            // contradict itself.
            unspoken @ (Self::NoSuchNode { .. } | Self::NotASpace { .. }) => unspoken,
            Self::NotABody { node } => Self::NotABody { node: again(node) },
            Self::InTheWorld { node } => Self::InTheWorld { node: again(node) },
            Self::NotEvaluated { node } => Self::NotEvaluated { node: again(node) },
            Self::NoSpace { node } => Self::NoSpace { node: again(node) },
            Self::MovedSpace { node, into } => Self::MovedSpace {
                node: again(node),
                into: Box::new(doc.spoken_space(into.id())),
            },
            // The space is said as the document that last held it said
            // it.
            Self::NoSuchSpace { node, space } => Self::NoSuchSpace {
                node: again(node),
                space,
            },
            Self::NotShown { node, space } => Self::NotShown {
                node: again(node),
                space: Box::new(doc.spoken_space(space.id())),
            },
        }
    }
}

impl DisplayFault {
    /// This fault with its nodes spoken from `doc`
    /// ([`AdmissionFault::respoken`]).
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        match self {
            Self::Admission(fault) => Self::Admission(fault.respoken(doc)),
            unspoken @ (Self::NonRigidFrame { .. }
            | Self::NoFreeMove
            | Self::FreeMoveInFlight
            | Self::WrongFreeMove) => unspoken,
        }
    }
}

impl From<AdmissionFault> for DisplayFault {
    fn from(fault: AdmissionFault) -> Self {
        Self::Admission(fault)
    }
}

/// A typed display-state refusal (closed enum, D4 ¶3). Every arm names
/// its subject; none is a message composed about another layer's
/// failure.
///
/// # Two families, and the split is a type
///
/// [`DisplayFault::Admission`] carries the faults that say a node
/// cannot hold display state — the answers of the tests
/// [`display_check`], [`free_move_check`] and [`space_check`], each
/// naming the id it is about. The four arms beside it are about a
/// gesture or a frame and name no id at all.
///
/// The families are apart because one of them is joined into a list
/// and the other is not: [`DisplayState::prune`] fills every
/// [`Withdrawn`] from [`display_check`], [`space_check`] and the
/// gesture's own check, so a
/// withdrawal's cause is an [`AdmissionFault`] by signature rather than
/// by a property of those functions' error sets, and
/// [`AdmissionFault`]'s own docs carry what the join then needs of its
/// sentences.
///
/// A caller rendering these must not promise its reader one
/// vocabulary across all of them.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayFault {
    /// A body or a space the document cannot admit a display operation on.
    ///
    /// The whole of [`AdmissionFault`], forwarded: every door here
    /// runs one of the admission tests, and the fault it raises is
    /// the test's own answer rather than a re-wording of it.
    Admission(AdmissionFault),
    /// The previewed frame is not one the document admits
    /// ([`Frame::admission_fault`]: a finite rigid motion). Refused because a
    /// scaling or mirroring probe would draw geometry the document
    /// cannot mean, and because the pick path compares hit distances
    /// across bodies, which only lengths-preserving frames keep
    /// comparable.
    NonRigidFrame {
        /// The offending frame's determinant (NaN when non-finite).
        determinant: f64,
    },
    /// A free-move gesture operation arrived with no gesture in
    /// flight.
    NoFreeMove,
    /// A free-move gesture is already in flight.
    FreeMoveInFlight,
    /// A free-move operation named a node that is not the one being
    /// probed — a preview or a commit for a node other than the one the
    /// open probe was begun on.
    ///
    /// [`crate::session::Refusal::WrongGesture`]'s twin on this drag,
    /// and separate from [`DisplayFault::FreeMoveInFlight`] for its
    /// reason: that fault answers a second BEGIN, this one answers a
    /// driving operation whose subject is not the probe it reaches.
    WrongFreeMove,
}

impl core::fmt::Display for DisplayFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Admission(fault) => write!(f, "{fault}"),
            Self::NonRigidFrame { determinant } => write!(
                f,
                "the free-move frame is not a finite rigid motion (determinant {determinant}); \
                 the probe admits rotations and translations only"
            ),
            Self::NoFreeMove => write!(f, "no free-move is in progress"),
            Self::FreeMoveInFlight => write!(f, "finish the free-move first"),
            Self::WrongFreeMove => write!(f, "that is not the free-move in progress"),
        }
    }
}

impl core::error::Error for DisplayFault {}

/// **`node` is a live node that builds a body**, or the typed refusal
/// that says which of the two ways it is not — the admission a body's
/// hide asks ([`DisplayState::set_body_hidden`]) and the first half of
/// the free-move's ([`free_move_check`]).
///
/// Any body is admitted, drawn or not: a hide is the state of the
/// body's toggle, and a body the default picture leaves out (an operand
/// a boolean took over) keeps its toggle's state for the moment it is
/// shown.
///
/// **The two refusals are spelled apart** for
/// [`AdmissionFault::NoSuchNode`]'s reason: an absent id and a node
/// that builds no body are different news at the doors that ANSWER a
/// user's operation.
///
/// # Errors
///
/// [`AdmissionFault::NoSuchNode`], [`AdmissionFault::NotABody`].
pub fn body_check(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Result<(), AdmissionFault> {
    match doc.node(node) {
        Some(_) if doc.node_space(node).is_some() => Ok(()),
        Some(_) => Err(AdmissionFault::NotABody {
            node: doc.spoken(node),
        }),
        None => Err(AdmissionFault::NoSuchNode {
            node: doc.spoken(node),
        }),
    }
}

/// **`space` is a live space of the document** — the admission a
/// space's hide asks ([`DisplayState::set_space_hidden`]). Any space is
/// admitted, shown or not, for [`body_check`]'s reason.
///
/// # Errors
///
/// [`AdmissionFault::NotASpace`].
pub fn space_hide_check(doc: &Doc<ProfileProgram>, space: VarId) -> Result<(), AdmissionFault> {
    match doc.var(space).map(pncad::document::Var::kind) {
        Some(VarKind::Space) => Ok(()),
        _ => Err(AdmissionFault::NotASpace {
            space: Box::new(doc.spoken_space(space)),
        }),
    }
}

/// Every node in `node`'s consuming-edge ancestry, `node` itself
/// included — "which nodes' work went into this drawn thing".
fn ancestry(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> BTreeSet<RecipeNodeId> {
    let mut seen = BTreeSet::new();
    let mut stack = vec![node];
    while let Some(id) = stack.pop() {
        if !seen.insert(id) {
            continue;
        }
        stack.extend(geometry_upstream(doc, id));
    }
    seen
}

/// `node`'s upstream ([`Doc::upstream`]) less the nodes it reaches only
/// through a constraint's side: a constraint reads where another copy
/// stands, not material, so a copy mated to an instance draws none of
/// that instance's geometry.
fn geometry_upstream(doc: &Doc<ProfileProgram>, id: RecipeNodeId) -> Vec<RecipeNodeId> {
    let Some(node) = doc.node(id) else {
        return Vec::new();
    };
    let (sides, material): (Vec<_>, Vec<_>) = node
        .operand_rows()
        .into_iter()
        .partition(|(seat, _)| matches!(seat, OperandSlot::Side { .. }));
    let reached = |rows: Vec<(OperandSlot, VarId)>| -> BTreeSet<RecipeNodeId> {
        rows.into_iter()
            .flat_map(|(_, var)| doc.read_ports(var))
            .map(|(at, _)| at)
            .collect()
    };
    let through_sides = reached(sides);
    let through_material = reached(material);
    doc.upstream(id)
        .into_iter()
        .filter(|at| !through_sides.contains(at) || through_material.contains(at))
        .collect()
}

/// **Whether `node`'s geometry derives from `source`** — `source` in
/// `node`'s consuming-edge ancestry, `node` itself included.
///
/// [`drawn_deriving_from`]'s question asked of ONE pair, over the same
/// walk. What it is for: an entity minted at `source` and carried
/// upward reaches `node`'s output, so a node whose op leaves no trace
/// in a name — one that contributes no role segment — can still be
/// told which drawn entities passed through it.
pub fn derives_from(doc: &Doc<ProfileProgram>, node: RecipeNodeId, source: RecipeNodeId) -> bool {
    ancestry(doc, node).contains(&source)
}

/// **Every drawn body whose geometry derives from `node`**
/// ([`crate::world::drawn_bodies`]) — the node itself when it is drawn,
/// and every drawn body that reaches it through consuming edges
/// otherwise.
///
/// The answer to "if I am looking at this recipe node, what in the
/// picture is it responsible for". A node that draws nothing on its
/// own — a profile — has a non-empty answer here, which is exactly the
/// case that makes the question worth asking: the profile IS the shape
/// of the walls the extrude above it drew. A body fusing several
/// nodes' geometry is listed for each of them.
pub fn drawn_deriving_from(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
) -> BTreeSet<RecipeNodeId> {
    crate::world::drawn_bodies(doc)
        .into_iter()
        .map(|drawn| drawn.node)
        .filter(|&drawn| ancestry(doc, drawn).contains(&node))
        .collect()
}

/// **The free-move admission test** (DI5): the space `node`'s body is
/// in, its `Space` variable — any shown space but the world's.
///
/// A free-move moves the picture of a shown space, so the test reads
/// the space `node`'s body is in off the recipe ([`Doc::node_space`]):
/// every body drawn in it moves as one, whatever kind of node drew it.
/// A body in the world refuses ([`AdmissionFault::InTheWorld`]): the
/// world is the frame the other spaces move against, drawn where the
/// document puts it, and a frame on it would be the camera a second
/// time.
///
/// A body with no value has no picture to move, so the test also asks
/// an evaluation (DI3): `eval` answers `doc`, the session's
/// [`crate::session::DocSession::space_evaluation`], or is `None` when
/// none has landed that does ([`AdmissionFault::NotEvaluated`]).
///
/// # Errors
///
/// [`AdmissionFault::NoSuchNode`], [`AdmissionFault::NotABody`],
/// [`AdmissionFault::NotEvaluated`], [`AdmissionFault::NoSpace`],
/// [`AdmissionFault::InTheWorld`], [`AdmissionFault::NotShown`].
pub fn free_move_check(
    doc: &Doc<ProfileProgram>,
    eval: Option<&Evaluation<f64>>,
    node: RecipeNodeId,
) -> Result<VarId, AdmissionFault> {
    body_check(doc, node)?;
    let eval = eval.ok_or_else(|| AdmissionFault::NotEvaluated {
        node: doc.spoken(node),
    })?;
    match probe_space_of(doc, Some(eval), node) {
        // A space no picture shows has nothing to move.
        Ok(space) => space_check(doc, space, node).map(|()| space),
        Err(Unplaced::Refused) => Err(AdmissionFault::NoSpace {
            node: doc.spoken(node),
        }),
        Err(Unplaced::World) => Err(AdmissionFault::InTheWorld {
            node: doc.spoken(node),
        }),
    }
}

/// **Whether `space` still names a shown space** — the test a held
/// frame is re-asked when the document moves ([`DisplayState::prune`]),
/// `node` the one whose probe moves it. The answer is the recipe's
/// ([`crate::world::shown_spaces`]), so no evaluation is asked.
///
/// # Errors
///
/// [`AdmissionFault::NoSuchSpace`], [`AdmissionFault::NotShown`].
pub fn space_check(
    doc: &Doc<ProfileProgram>,
    space: VarId,
    node: RecipeNodeId,
) -> Result<(), AdmissionFault> {
    if doc.var(space).is_none() {
        return Err(AdmissionFault::NoSuchSpace {
            node: doc.spoken(node),
            space: Box::new(doc.spoken_space(space)),
        });
    }
    if crate::world::shown_spaces(doc).contains(&space) {
        Ok(())
    } else {
        Err(AdmissionFault::NotShown {
            node: doc.spoken(node),
            space: Box::new(doc.spoken_space(space)),
        })
    }
}

/// **Whether a held gesture still moves the space its node's body is
/// in** — the in-flight half of [`DisplayState::prune`]. The gesture
/// names its node and moves its space, so both are asked: the node is
/// still in the document and its body still in that space, the space
/// is still shown ([`space_check`]), and, with an evaluation, the body
/// still evaluates.
fn gesture_check(
    doc: &Doc<ProfileProgram>,
    eval: Option<&Evaluation<f64>>,
    probe: Probe,
    answers: Answers,
) -> Result<(), AdmissionFault> {
    let Probe { space, node } = probe;
    if doc.node(node).is_none() {
        return Err(AdmissionFault::NoSuchNode {
            node: doc.spoken(node),
        });
    }
    space_check(doc, space, node)?;
    match probe_space_of(doc, eval, node) {
        Ok(found) if found == space => Ok(()),
        Ok(found) => Err(AdmissionFault::MovedSpace {
            node: doc.spoken(node),
            into: Box::new(doc.spoken_space(found)),
        }),
        Err(Unplaced::World) => Err(AdmissionFault::MovedSpace {
            node: doc.spoken(node),
            into: Box::new(doc.spoken_space(VarId::WORLD)),
        }),
        Err(Unplaced::Refused) if answers == Answers::Structural => Ok(()),
        Err(Unplaced::Refused) => Err(AdmissionFault::NoSpace {
            node: doc.spoken(node),
        }),
    }
}

/// **Which of an evaluation's answers a prune acts on.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Answers {
    /// Every one: a landing of the committed document.
    All,
    /// Every one but a refused placement: a landing of a value
    /// gesture's scratch, which shares the committed graph but not its
    /// values ([`DisplayState::prune_structural`]). Where a copy is
    /// placed is the graph's, so the scratch answers it for the
    /// committed document too. A refusal may be a value's, and the
    /// evaluation does not say which decided it, so every refusal waits
    /// for a committed landing — a graph-decided one too (two mates
    /// that over-constrain a copy), which is then said at the drag's
    /// end rather than mid-drag.
    Structural,
}

/// Why a node's copy is in no space a free-move moves.
enum Unplaced {
    /// It is placed into the world's space.
    World,
    /// It names no live space, or `eval` gave it no value, so it has no
    /// picture.
    Refused,
}

/// **The space `node`'s copy is placed into**, read off the recipe
/// ([`Doc::node_space`]), the world's refused, and, with `eval`, a copy
/// the evaluation gave no value refused.
fn probe_space_of(
    doc: &Doc<ProfileProgram>,
    eval: Option<&Evaluation<f64>>,
    node: RecipeNodeId,
) -> Result<VarId, Unplaced> {
    let space = doc.node_space(node).ok_or(Unplaced::Refused)?;
    if eval.is_some_and(|eval| eval.value(node).is_none()) {
        return Err(Unplaced::Refused);
    }
    if space == VarId::WORLD {
        Err(Unplaced::World)
    } else {
        Ok(space)
    }
}

/// **The probe's words for the three G1 states**, declared once and
/// handed to [`g1::Slot`] at every door.
///
/// The machine is shared with the value drag and the vocabularies are
/// not: these three sentences are about a space's probe, and the
/// value drag's three are about a field's drag
/// ([`crate::session::refuse::Refusal`]).
fn free_move_words() -> g1::Refusals<DisplayFault> {
    g1::Refusals {
        none: DisplayFault::NoFreeMove,
        in_flight: DisplayFault::FreeMoveInFlight,
        wrong: DisplayFault::WrongFreeMove,
    }
}

/// What the scene and pick paths read: the display state snapshotted
/// as a value, in BOTH keyings — the raw facts as the chrome's toggles
/// hold them (hidden bodies, hidden spaces, a frame per space) and their
/// resolution onto the drawn bodies ([`crate::world::drawn_bodies`]) the
/// scene and the pick index are built from. A body's hide reaches what
/// draws it — the body itself, or for a `⊔` its pieces — and a space's
/// hide and frame every body drawn in that space.
///
/// Owned rather than borrowed so a caller can hold one across the
/// mutation that would invalidate a borrow, and because the maps are
/// a handful of entries.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayView {
    /// Bodies the author hid, by the node building each (the chrome's
    /// reading).
    pub hidden_bodies: BTreeSet<RecipeNodeId>,
    /// Spaces the author hid (the chrome's reading).
    pub hidden_spaces: BTreeSet<VarId>,
    /// Space → the display frame composed over its drawn bodies. An
    /// in-flight preview overrides that space's committed value.
    pub moved: BTreeMap<VarId, Frame>,
    /// The drawn bodies the hides reach — what the scene and the pick
    /// index drop ([`DisplayView::hides`]).
    pub hidden_drawn: Vec<crate::world::Drawn>,
    /// Drawn node → the probe frame governing it — what the scene
    /// displaces and marks, and the pick carries rays into.
    pub moved_nodes: BTreeMap<RecipeNodeId, Frame>,
}

impl DisplayView {
    /// The view that hides nothing and moves nothing — what every
    /// non-assembly consumer reads.
    pub fn none() -> Self {
        Self::default()
    }

    /// Whether the picture drops body `body` of `node`.
    pub fn hides(&self, node: RecipeNodeId, body: u32) -> bool {
        self.hidden_drawn
            .iter()
            .any(|drawn| drawn.draws(node, body))
    }
}

/// **What a display fact was held on**: a body, by the node building
/// it, or a space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    /// A body's hide, or the node a free-move was opened on.
    Node(RecipeNodeId),
    /// A space's hide.
    Space(VarId),
}

/// **One display fact the document withdrew**, and the typed fault
/// that says why it can no longer hold.
///
/// [`DisplayState::prune`] decides what to drop by asking a display
/// predicate, and the predicate answers an [`AdmissionFault`] whose
/// `Display` already names the cause and the remedy. Carrying the
/// answer instead of testing it with `is_ok` is what lets the chrome
/// render the fault through its own `Display` rather than compose
/// prose about it: the cause is the whole content of the sentence, and
/// it exists exactly once, here.
#[derive(Debug, Clone, PartialEq)]
pub struct Withdrawn {
    /// What the display fact was taken on: the body or the space
    /// hidden, the node a killed gesture was opened on, and, for a
    /// discarded frame — which is keyed by its space — the node whose
    /// gesture committed it last.
    pub held: Held,
    /// Why the document no longer admits it.
    ///
    /// **[`AdmissionFault`] and not [`DisplayFault`]**, because the
    /// causes a prune can withdraw on are exactly what its tests
    /// ([`body_check`], [`space_hide_check`], [`space_check`] and the
    /// gesture's, which asks its own body the same space question)
    /// answer — and because [`crate::frame::Withdrawal`] joins several
    /// of these into one sentence with
    /// [`crate::frame::LIST_SEPARATOR`], which is a claim about what
    /// those sentences can say.
    pub cause: AdmissionFault,
}

/// **What a [`DisplayState::prune`] withdrew**, per kind of display
/// fact — the report a caller turns into what the user reads.
///
/// # Three facts, not one
///
/// A discarded probe, a dropped hide and a killed gesture are all
/// display state the document stopped admitting, and they are NOT the
/// same news:
///
/// - A **supersession** is a substitution. The user's hand placement
///   was an answer to "where does this part go", and the pin that
///   landed is a better answer to the same question — the document now
///   says where the part goes, and the probe steps aside for it. The
///   picture loses the probe treatment and keeps the part.
/// - A **dropped hide** is not superseded by anything. The user asked
///   for a body or a space not to be drawn, and the document did not
///   answer that question differently; it made the question unaskable —
///   the body or the space is gone, and nothing takes the hide's place.
/// - A **killed gesture** is neither. It is the drag the user's hand
///   is still on, and nothing substituted for it: the placement it
///   would have landed was never asked of the document, so there is no
///   better answer to step aside for. The picture loses a preview the
///   user was steering. It is also the only one of the three that can
///   happen at most once, because at most one free-move gesture is in
///   flight.
///
/// So they are ranked together and worded apart. Nothing here composes
/// any of the three sentences: the fields carry the faults, and the
/// chrome renders each one through its own `Display`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PruneReport {
    /// Spaces whose COMMITTED free-move frame was discarded, each by
    /// the node whose gesture committed it, with the [`space_check`] fault
    /// that discarded it — the space no longer shown, or gone.
    ///
    /// **Only committed probes.** A gesture in flight when the
    /// transition lands dies too; it is `killed_gesture`, because it
    /// is not a supersession.
    pub superseded: Vec<Withdrawn>,
    /// Bodies and spaces whose HIDE was dropped, each with the
    /// [`body_check`] or [`space_hide_check`] fault that dropped it:
    /// the document no longer holds what was hidden.
    pub dropped_hides: Vec<Withdrawn>,
    /// The in-flight free-move gesture this prune killed, by the
    /// node it was opened on, with the fault that killed it — the
    /// node gone, or its body no longer in the space the gesture moves — or
    /// `None`, which is
    /// both "no gesture was in flight" and "the one in flight still
    /// holds". The two are the same news to a reader: nothing was
    /// taken.
    ///
    /// **`Option` rather than a `Vec`**, because a [`DisplayState`]
    /// holds one free-move gesture and a prune can kill at most that
    /// one. A collection here would carry a plural no caller could
    /// produce and every reader would have to word.
    pub killed_gesture: Option<Withdrawn>,
}

impl PruneReport {
    /// Whether the prune withdrew nothing at all — the ordinary case,
    /// and the one a caller reports nothing for.
    ///
    /// Crate-private: [`DisplayState::prune`] reads it deciding whether
    /// the revision moved, and `DocSession::land` deciding whether a
    /// landing's prune is news to keep. The two types themselves are `pub`
    /// structurally rather than by choice — this is `prune`'s return
    /// type and [`Withdrawn`] is the element type of three public
    /// [`crate::session::OpOutcome`] fields, so neither could be
    /// narrower and both are re-exported to keep those fields
    /// nameable.
    ///
    /// **Destructured rather than field-read**, so a fourth kind of
    /// withdrawal is E0027 here rather than a withdrawal that leaves
    /// the revision where it was — which is the chrome not rebuilding
    /// a picture that changed.
    pub(crate) fn is_empty(&self) -> bool {
        let Self {
            superseded,
            dropped_hides,
            killed_gesture,
        } = self;
        superseded.is_empty() && dropped_hides.is_empty() && killed_gesture.is_none()
    }
}

/// **What a free-move gesture holds**: the space it moves, and the
/// node whose drawn body the gesture was opened on, which the preview
/// and the commit name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Probe {
    space: VarId,
    node: RecipeNodeId,
}

/// **A committed free-move frame**, and the node whose gesture
/// committed it, which a withdrawal names.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Moved {
    frame: Frame,
    node: RecipeNodeId,
}

/// **The one home** for hide and free-move state (the seam-friction
/// inventory discipline: no per-widget shadows). Owned by the session;
/// every mutation is a typed operation routed through it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayState {
    /// The bodies the author hid, by the node building each.
    hidden_bodies: BTreeSet<RecipeNodeId>,
    /// The spaces the author hid.
    hidden_spaces: BTreeSet<VarId>,
    /// The committed free-move frames, keyed by the space each was
    /// drawn for (DI5).
    moves: BTreeMap<VarId, Moved>,
    /// The free-move gesture in flight, if any.
    ///
    /// **Not spelled `gesture`.** [`crate::session::DocSession`] holds
    /// a `gesture` of its own — the VALUE drag a slot or parameter
    /// field opens — and the two are independent drags that can be
    /// open at once, so one field name must not stand for both. Why
    /// that overlap is sound, the identity it rests on, and why DI5
    /// keeps it so (a release writes nothing to the document) are
    /// stated at
    /// [`crate::session::SessionOp::permitted_during_value_gesture`].
    free_move: g1::Slot<Probe, Frame>,
    /// Bumped on every visible change — the chrome's cheap "does the
    /// drawn scene need rebuilding" key, beside the evaluation
    /// generation and δ.
    revision: u64,
}

impl DisplayState {
    /// Empty display state: nothing hidden, nothing probed.
    pub fn new() -> Self {
        Self::default()
    }

    /// The revision counter (see the field docs).
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Whether the body `node` builds is hidden.
    pub fn is_body_hidden(&self, node: RecipeNodeId) -> bool {
        self.hidden_bodies.contains(&node)
    }

    /// Whether `space` is hidden.
    pub fn is_space_hidden(&self, space: VarId) -> bool {
        self.hidden_spaces.contains(&space)
    }

    /// The hidden bodies, by the node building each.
    pub fn hidden_bodies(&self) -> &BTreeSet<RecipeNodeId> {
        &self.hidden_bodies
    }

    /// The hidden spaces.
    pub fn hidden_spaces(&self) -> &BTreeSet<VarId> {
        &self.hidden_spaces
    }

    /// The committed free-move frame of `space`, if any. The in-flight
    /// preview is NOT reported here — [`DisplayState::view`] is the door
    /// that composes both.
    pub fn free_move_of(&self, space: VarId) -> Option<&Frame> {
        self.moves.get(&space).map(|moved| &moved.frame)
    }

    /// The node a free-move gesture is currently probing, if one is in
    /// flight.
    pub fn probing(&self) -> Option<RecipeNodeId> {
        self.free_move.held().map(|probe| probe.node)
    }

    /// The snapshot the scene and pick paths consume, resolved onto
    /// `doc`'s drawn bodies ([`crate::world::drawn_bodies`]): a body's
    /// hide onto what draws it — the body, or for a `⊔` its pieces — a
    /// space's hide and frame onto every body drawn in the space
    /// ([`Doc::node_space`]).
    ///
    /// `eval` is the session's landed evaluation; what is drawn and
    /// where is the recipe's, so the view reads no evaluation and takes
    /// it only to keep the door's shape.
    pub fn view(&self, doc: &Doc<ProfileProgram>, eval: Option<&Evaluation<f64>>) -> DisplayView {
        let _ = eval;
        let mut moved: BTreeMap<VarId, Frame> = self
            .moves
            .iter()
            .map(|(&space, held)| (space, held.frame))
            .collect();
        if let Some((probe, &frame)) = self.free_move.previewing() {
            moved.insert(probe.space, frame);
        }
        let mut hidden_drawn = Vec::new();
        let mut moved_nodes = BTreeMap::new();
        for drawn in crate::world::drawn_bodies(doc) {
            let space = doc.node_space(drawn.node);
            if let Some(&frame) = space.and_then(|space| moved.get(&space)) {
                moved_nodes.insert(drawn.node, frame);
            }
            let hidden = self.hidden_bodies.contains(&drawn.node)
                || drawn
                    .within
                    .iter()
                    .any(|at| self.hidden_bodies.contains(at))
                || space.is_some_and(|space| self.hidden_spaces.contains(&space));
            if hidden {
                hidden_drawn.push(drawn);
            }
        }
        DisplayView {
            hidden_bodies: self.hidden_bodies.clone(),
            hidden_spaces: self.hidden_spaces.clone(),
            moved,
            hidden_drawn,
            moved_nodes,
        }
    }

    /// Hide or show the body `node` builds.
    ///
    /// # Errors
    ///
    /// [`body_check`]'s: [`AdmissionFault::NoSuchNode`] and
    /// [`AdmissionFault::NotABody`].
    pub fn set_body_hidden(
        &mut self,
        doc: &Doc<ProfileProgram>,
        node: RecipeNodeId,
        hidden: bool,
    ) -> Result<(), DisplayFault> {
        body_check(doc, node)?;
        let changed = if hidden {
            self.hidden_bodies.insert(node)
        } else {
            self.hidden_bodies.remove(&node)
        };
        if changed {
            self.revision += 1;
        }
        Ok(())
    }

    /// Hide or show `space`, every body drawn in it.
    ///
    /// # Errors
    ///
    /// [`space_hide_check`]'s: [`AdmissionFault::NotASpace`].
    pub fn set_space_hidden(
        &mut self,
        doc: &Doc<ProfileProgram>,
        space: VarId,
        hidden: bool,
    ) -> Result<(), DisplayFault> {
        space_hide_check(doc, space)?;
        let changed = if hidden {
            self.hidden_spaces.insert(space)
        } else {
            self.hidden_spaces.remove(&space)
        };
        if changed {
            self.revision += 1;
        }
        Ok(())
    }

    /// Open a free-move gesture on the space `node`'s body is in (DI5),
    /// as [`free_move_check`] reads it.
    ///
    /// The in-flight refusal and the order — admission checked only
    /// once the probe slot is known free — are [`g1::Slot::begin`]'s,
    /// held there for both gestures.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::FreeMoveInFlight`], and [`free_move_check`]'s
    /// refusals.
    pub fn begin_free_move(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: Option<&Evaluation<f64>>,
        node: RecipeNodeId,
    ) -> Result<(), DisplayFault> {
        self.free_move.begin(free_move_words(), || {
            let space = free_move_check(doc, eval, node)?;
            Ok(Probe { space, node })
        })
    }

    /// Stream one preview frame into the in-flight gesture, replacing
    /// the last: the composed display value is `frame`, never an
    /// accumulation of deltas. The replacement and the two refusals are
    /// [`g1::Slot::preview`]'s; what is this door's own is the
    /// rigid-motion check.
    ///
    /// **It names the node it is probing**, and the name is checked
    /// before the frame is: a drag on a second body's field cannot
    /// compose its frame onto the probe another body's drag holds. [`DisplayFault::WrongFreeMove`] carries the argument.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`], [`DisplayFault::WrongFreeMove`],
    /// [`DisplayFault::NonRigidFrame`].
    pub fn preview_free_move(
        &mut self,
        node: RecipeNodeId,
        frame: Frame,
        tol: pncad::geom_core::Tol,
    ) -> Result<(), DisplayFault> {
        self.free_move.preview(
            free_move_words(),
            |probed| probed.node == node,
            |_| {
                if frame.admission_fault(tol).is_none() {
                    Ok((frame, ()))
                } else {
                    Err(DisplayFault::NonRigidFrame {
                        determinant: frame.determinant(),
                    })
                }
            },
        )?;
        self.revision += 1;
        Ok(())
    }

    /// Land the gesture: its last previewed frame becomes the committed
    /// frame of the space it moves, keyed by the space. It
    /// writes nothing to the document (DI5). The no-move rule — a gesture
    /// that never previewed lands nothing — and the name check that
    /// leaves a probe this commit does not name in flight are
    /// [`g1::Slot::commit`]'s, which is where they are held for both
    /// gestures; this door lands what it hands back.
    ///
    /// A bit-exact identity commit REMOVES the entry: "probed to
    /// exactly where the document draws it" is the same picture as
    /// "not probed", and the distinctness treatment must not mark a
    /// part that is not displaced.
    ///
    /// Names its node for [`DisplayState::preview_free_move`]'s
    /// reason.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`], [`DisplayFault::WrongFreeMove`].
    pub fn commit_free_move(&mut self, node: RecipeNodeId) -> Result<(), DisplayFault> {
        let landed = self
            .free_move
            .commit(free_move_words(), |probed| probed.node == node)?;
        if let Some((probed, frame)) = landed {
            if frame.is_identity_bits() {
                self.moves.remove(&probed.space);
            } else {
                self.moves.insert(
                    probed.space,
                    Moved {
                        frame,
                        node: probed.node,
                    },
                );
            }
            self.revision += 1;
        }
        Ok(())
    }

    /// Abandon the gesture, restoring the committed picture. The
    /// revision moves only for a probe that had previewed, which is
    /// what [`g1::Slot::cancel`] answers: one that never moved put
    /// nothing on screen to take off it.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`].
    pub fn cancel_free_move(&mut self) -> Result<(), DisplayFault> {
        if self.free_move.cancel(free_move_words())? {
            self.revision += 1;
        }
        Ok(())
    }

    /// Reconcile this state with a document: DISCARD every committed
    /// frame whose space is no longer shown (it holds no drawn body, or
    /// it is gone; DI5), drop hidden entries whose body or space is
    /// gone, and kill an in-flight gesture whose node is gone or whose
    /// body is no longer in the space it moves.
    ///
    /// `eval` is an evaluation of `doc` itself, or `None`. The session
    /// prunes twice: when the document changes, with no evaluation,
    /// which withdraws what the document alone says — every frame
    /// ([`space_check`]), and a gesture whose copy is gone or placed
    /// elsewhere — and when the evaluation answering it lands, which
    /// withdraws a gesture whose copy no longer evaluates. A landing of
    /// a value gesture's scratch prunes through
    /// [`DisplayState::prune_structural`] instead.
    ///
    /// **Returns the [`PruneReport`]** — every withdrawn display fact,
    /// each carrying the fault that withdrew it, so a caller reports
    /// what happened and why rather than inferring either. The faults
    /// come from the same predicates that decide: the decision and its
    /// explanation are one value, and there is no second place for
    /// them to disagree.
    ///
    /// A killed in-flight gesture is in neither LIST, because it
    /// committed nothing and there can be only one of it; it is the
    /// report's [`PruneReport::killed_gesture`], carrying the same
    /// kind of fault as the other two.
    pub fn prune(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: Option<&Evaluation<f64>>,
    ) -> PruneReport {
        self.prune_with(doc, eval, Answers::All)
    }

    /// [`DisplayState::prune`] against a value gesture's scratch and the
    /// evaluation of it that landed, acting on what the node graph
    /// decides — a re-target, a node gone — and not on a
    /// refused placement: the scratch shares the committed graph, so its
    /// landing answers the committed document's structure, and a
    /// preview the user may still cancel discards no committed display
    /// state over a value. A refusal is not told apart by what decided
    /// it, so a graph-decided one waits too ([`Answers::Structural`]).
    ///
    /// The session runs it when a scratch landing overtakes a committed
    /// edit's own landing (a pin committed, then a drag previewed before
    /// the pin landed), so the pin takes effect mid-drag rather than
    /// after the drag ends.
    pub fn prune_structural(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
    ) -> PruneReport {
        self.prune_with(doc, Some(eval), Answers::Structural)
    }

    /// [`DisplayState::prune`]'s body, over which answers it acts on.
    fn prune_with(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: Option<&Evaluation<f64>>,
        answers: Answers,
    ) -> PruneReport {
        let mut superseded = Vec::new();
        self.moves
            .retain(|&space, held| match space_check(doc, space, held.node) {
                Ok(()) => true,
                Err(cause) => {
                    superseded.push(Withdrawn {
                        held: Held::Node(held.node),
                        cause,
                    });
                    false
                }
            });
        let dropped_hides: Vec<Withdrawn> = self
            .hidden_bodies
            .iter()
            .filter_map(|&node| {
                body_check(doc, node).err().map(|cause| Withdrawn {
                    held: Held::Node(node),
                    cause,
                })
            })
            .chain(self.hidden_spaces.iter().filter_map(|&space| {
                space_hide_check(doc, space).err().map(|cause| Withdrawn {
                    held: Held::Space(space),
                    cause,
                })
            }))
            .collect();
        for dropped in &dropped_hides {
            match dropped.held {
                Held::Node(node) => self.hidden_bodies.remove(&node),
                Held::Space(space) => self.hidden_spaces.remove(&space),
            };
        }
        // The fault is CARRIED, not tested: `is_err()` here would
        // throw away the one value that says why the drag under the
        // user's hand stopped, at the instant it is computed.
        let killed_gesture = self.free_move.held().and_then(|probe| {
            gesture_check(doc, eval, *probe, answers)
                .err()
                .map(|cause| Withdrawn {
                    held: Held::Node(probe.node),
                    cause,
                })
        });
        if killed_gesture.is_some() {
            self.free_move.discard();
        }
        let report = PruneReport {
            superseded,
            dropped_hides,
            killed_gesture,
        };
        if !report.is_empty() {
            self.revision += 1;
        }
        report
    }

    /// Forget everything — what opening a different document does.
    ///
    /// **Destructured rather than field-cleared**, so a field added to
    /// [`DisplayState`] is E0027 here rather than silently surviving
    /// the door that opens a different document.
    ///
    /// `revision` is the field the walk does not forget, and the one
    /// the pattern exists to keep visible: it is the chrome's rebuild
    /// key, so it is bumped when the reset was visible and never reset
    /// itself — a counter that went backwards would name a picture the
    /// chrome has already drawn.
    ///
    /// # Why this door returns no [`PruneReport`], and its sibling does
    ///
    /// This takes MORE than [`DisplayState::prune`] ever does — every
    /// hide and every placement, unconditionally — and says nothing
    /// about any of it. That asymmetry is decided here rather than
    /// left to be read as an oversight, and it turns on the two doors
    /// withdrawing display state for different reasons.
    ///
    /// **A prune's withdrawals are a SIDE EFFECT of an act about
    /// something else.** The user mated two parts, or undid a step;
    /// losing a hand placement was no part of what they asked for, so
    /// the loss is news and its cause is the whole content of the
    /// sentence. **A replacement is the act itself.** `Open` and
    /// `NewDocument` change what the session is about; display state
    /// is state of a session over ONE document (G3 — it is never
    /// persisted, and "save, reopen, layer-3 state gone" is a property
    /// of the structure), so a user who replaces the document has
    /// asked for exactly this. A per-body notice would report the
    /// act back to the person who performed it.
    ///
    /// **And a truthful [`Withdrawn`] cannot be built here at all**,
    /// which is what makes this a typing fact rather than a taste in
    /// wording. A `Withdrawn` carries an [`AdmissionFault`] about a
    /// document, and the only document left to ask is the replacement
    /// — where these ids mean something else. A [`RecipeNodeId`] is
    /// minted from a counter the `Doc` owns, so the outgoing
    /// document's id 3 and the incoming document's id 3 are unrelated
    /// nodes. Asking [`space_check`] about them would answer `Ok(())`
    /// wherever the incoming document happens to show a space at that
    /// id — a report that says nothing was withdrawn
    /// while everything was — and [`AdmissionFault::NoSuchNode`]
    /// otherwise, which is a true sentence about the incoming document
    /// and a false explanation of where the placement went.
    ///
    /// **The in-flight gesture is answered the other way, and not
    /// here.** A drag dissolved under the pointer is the half-acted
    /// state a REFUSAL at the door exists to prevent, which is what
    /// the value gesture already got; the argument above is why a
    /// report could not have been the answer for it either. So the two
    /// doors that reach this one refuse while a free move is in flight
    /// ([`crate::session::SessionOp::permitted_during_free_move`]),
    /// so a session reaches this with `free_move` already `None`. It
    /// is still cleared below, unconditionally: this is a `pub` door
    /// on a `pub` value and its promise is to forget EVERYTHING, which
    /// a caller holding a [`DisplayState`] of its own is entitled to
    /// without a session in front of it.
    pub fn clear(&mut self) {
        let Self {
            hidden_bodies,
            hidden_spaces,
            moves,
            free_move,
            revision,
        } = self;
        if !hidden_bodies.is_empty()
            || !hidden_spaces.is_empty()
            || !moves.is_empty()
            || free_move.held().is_some()
        {
            *revision += 1;
        }
        hidden_bodies.clear();
        hidden_spaces.clear();
        moves.clear();
        free_move.discard();
    }
}
