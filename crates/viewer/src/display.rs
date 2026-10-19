//! Layer-3 display state for assemblies: per-instance **hide** and the
//! **free-move** fit probe (GUI-DESIGN G3), each with exactly one home.
//!
//! # What this state is, and what it is not
//!
//! Both facts here are about the PICTURE, never about the model. A
//! hidden instance stays in the document and in the feature tree; only
//! the drawn scene and the pick index drop it. A free-move transform is
//! the G3 fit probe — a display frame composed over a loose space's
//! drawn copies so a user can hold a part against another BEFORE
//! authoring the mate — and it involves no solver, enters no history, and is
//! never persisted: `save` writes the document, and a reopened session
//! starts with this state empty. That is not an accident of the code
//! path; it is G3's boundary, and the round-trip row pins it. DI5
//! (`crates/editor-core/IDENTITY.md`) states the rule over loose spaces:
//! a free-move moves the picture of a loose space, its release writes
//! nothing to the document, and the frame is keyed by the space's root
//! and dropped by [`DisplayState::prune`] once that root stops naming a
//! loose space. Keeping a position means stating it, as a placement's
//! mates and typed values.
//!
//! # Eligibility is read off the evaluation
//!
//! Free-move admits a **loose space** (D10): a copy whose bundle pins
//! less than its body needs, with every copy pinned to it. Which space
//! a copy lives in is the solve's answer ([`Evaluation::space`]), so
//! the admission test ([`free_move_check`]) reads it off an evaluation
//! that answers the document, and refuses typed while none has landed
//! ([`AdmissionFault::NotEvaluated`]) rather than waiting. A copy the
//! world pins, or one mated onto a copy in the world, lives in the
//! world's space and refuses ([`AdmissionFault::InTheWorld`]): its pose
//! is the document's, and a display value contradicting it would draw
//! a relation the document does not have. Two copies mated to each
//! other share a space, admit to its one root, and move as one.
//!
//! # Hides name instances; the picture is drawn per copy
//!
//! The scene and the pick index emit geometry per WORLD PLACEMENT's
//! copy (A10), and a placement reads whatever body it places — an
//! instance, or a `Pattern` or `Part` over one. A hide
//! therefore PROPAGATES: it names the instance, and [`drawn_targets`]
//! resolves it to every placement whose geometry derives from that
//! instance alone (hiding a patterned instance hides all its placed
//! copies — the pattern replicates the instance, and the display fact
//! is the instance's). A free-move frame instead moves every placement
//! in its space ([`DisplayView::moved_placements`]): a pattern's copies
//! placed against the world are in the world's space and stay put. A
//! placement whose body fuses SEVERAL instances' geometry (a
//! cross-instance boolean) can be addressed by none of them separately,
//! and both operations refuse typed ([`AdmissionFault::FusedGeometry`])
//! — the alternative, accepting the op and drawing nothing different,
//! is the silent no-op G3's honesty rule forbids. The free-move refuses
//! it so that it answers the test the properties pane draws its control
//! from. A frame already held when a fuse arrives stays: its space is
//! still loose and the frame still moves that space's own copies, while
//! the fused product is placed by its own placement, in its own space,
//! and does not move with it.
//!
//! # Supersession: the frame goes when its root stops naming a loose space
//!
//! When an edit pins a free-moved space to the world, puts its root in
//! another space, leaves its placement refused, or removes its root,
//! its frame is **discarded** —
//! removed outright, not zeroed and kept — by [`DisplayState::prune`].
//! Discard rather than zero because the value's LEGALITY is derived
//! from the document and the evaluation that answers it: a frame for a
//! space that no longer exists is unrepresentable state, not a
//! transform that happens to be identity. Its copies are thereafter
//! drawn at their solved placements, undistinguished, which is exactly
//! the honesty rule: the probe treatment marks "this is not where the
//! document puts it", and after the pin lands the document DOES put it
//! there.
//!
//! The session prunes twice for one edit. The edit's own prune has no
//! evaluation of the new document and asks only what the document says
//! (the root is gone); between the edit and its landing the frame draws
//! over the last-known copies. The landing prunes again with the
//! evaluation that knows the spaces, and the session keeps what that
//! one withdraws for the chrome to say
//! (`DocSession::take_landed_withdrawals`).
//!
//! # A hide the picture can no longer honour is DROPPED, and said
//!
//! The same reconciliation drops a hidden entry whose instance the
//! picture can no longer address separately — the instance was
//! deleted, or its geometry became fused into a product with others.
//! That is not a supersession: nothing replaced the user's choice, it
//! stopped being expressible, and where the cause is a fuse the part
//! they hid is DRAWN AGAIN. [`DisplayState::prune`] therefore reports
//! it beside the discarded probes ([`PruneReport`]) rather than
//! undoing a user's choice in silence.
//!
//! # Which history holds a committed free-move: none
//!
//! The G1 preview/commit shape applies — a gesture streams preview
//! frames and lands exactly one committed value, and its three
//! transition rules are [`crate::g1::Slot`]'s, shared with the value
//! drag — but the commit replaces this state's entry and enters NO
//! history: the plan's undo note governs document state only, and no
//! layer-3 history exists in v1. Undo/redo therefore never change what is hidden or probed;
//! they can only DISCARD a probe by pinning its space, joining it to
//! another, leaving its placement refused, or removing its root.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::{BTreeMap, BTreeSet};

use pncad::document::{
    Doc, Evaluation, Frame, Node, OperandSlot, ProfileProgram, RecipeNodeId, Space, SpokenNode,
    VarId,
};
use pncad::geom_core::Vec3;

use crate::g1;

/// How far off exactly orthonormal a free-move frame's linear part may
/// be and still count as rigid.
///
/// A display bound, not a kernel predicate: nothing downstream decides
/// geometry on it. What it protects is the pick path — a hit's ray
/// parameter is compared across differently-moved instances, and that
/// comparison is only meaningful when every admitted frame preserves
/// lengths. Generous against accumulated rotation round-off, tight
/// against any actual scale or shear.
const RIGID_SLACK: f64 = 1e-9;

/// **Why an instance cannot hold display state** — what the admission
/// tests answer, and the only cause a [`Withdrawn`] carries.
///
/// # Each arm names its subject through the spoken node
///
/// The rule over the whole enum, not a property of one arm: an arm
/// says its subject as [`SpokenNode`] says it — always the node's kind
/// as the document holds it, with its label beside it when it has one
/// (`InstantiatePart N`, `Place "shelf" (N)`) — and never writes the
/// subject's kind in its own words. A frame's withdrawal asks about its
/// space's root, which is a placement and may be a `Place` rather than
/// an instance, so a sentence that wrote "instance" would be false there.
/// An id the document does not hold has no kind and reads **"node N"**
/// ([`AdmissionFault::NoSuchNode`]). [`AdmissionFault::NotAnInstance`]
/// is the one arm whose own words say "instance", because its whole
/// content is that the node is not one. The census in
/// `crates/viewer/tests/frame_policy.rs` holds the "instance" half of
/// the rule; the other kind words are held by review.
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
/// a guard on one (`MovedSpace` speaks two, by whether the copy left
/// into a space of its own), each sampled by the census. The
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
    /// **Spelled apart from [`AdmissionFault::NotAnInstance`]** because
    /// the two are different news to a user holding display state on
    /// the id: a wrong-kind node is a mis-aimed operation, and an
    /// absent one is the thing they were looking at being gone. The
    /// reconciliation in [`DisplayState::prune`] reports the fault it
    /// discards on, so the difference is the whole content of the
    /// sentence the status line then shows.
    ///
    /// **It is decided in [`instance_check`]**, which [`drawn_targets`]
    /// runs first, so the arm reaches every OPERATION downstream of it
    /// and not only the prune: [`DisplayState::set_hidden`] and the
    /// free-move admission ([`free_move_check`], and so
    /// [`DisplayState::begin_free_move`]). At those doors a user who
    /// aims a display operation at an id the document does not hold
    /// reads *node N is not in the document* where they read *node N is
    /// not a part instance* — the better sentence, because the id
    /// denotes nothing and saying only that it is not an instance
    /// implies something is there.
    ///
    /// **The properties panel runs the same test and renders neither
    /// arm, which is not a discard.** `PropertiesPane::instance_ui`
    /// draws no per-instance section for either refusal. For this arm
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
    /// The node is not an `InstantiatePart`, so it has no per-instance
    /// display state to set.
    NotAnInstance {
        /// The node named, as the document held it.
        node: SpokenNode,
    },
    /// The instance's copy lives in the world's space: its placement
    /// pins it there, by a world pin or a mate onto a copy in the world.
    /// Free-move moves the picture of a loose space (DI5) and a world
    /// copy is drawn where the document puts it, so the probe refuses.
    InTheWorld {
        /// The instance, as the document held it.
        instance: SpokenNode,
    },
    /// No evaluation of the current document has landed, so the space
    /// the instance's copy lives in is not known yet (DI3: an evaluation
    /// names the document it answers, and an older one's spaces are not
    /// this one's).
    NotEvaluated {
        /// The instance, as the document held it.
        instance: SpokenNode,
    },
    /// The evaluation gave the instance's placement no space: its value
    /// did not evaluate (the placement refused, or an input it reads
    /// failed), so there is no picture of it to move.
    NoSpace {
        /// The instance, as the document held it.
        instance: SpokenNode,
    },
    /// The copy now lives in another loose space than the one a frame or
    /// a gesture was drawn for: a mate put a frame's root in the space
    /// `into` roots, or an edit split a gesture's instance off into a
    /// space of its own (`into` is then the instance itself). The frame
    /// or gesture goes (DI5); the probe still admits the copy, to the
    /// space it lives in now. Raised by the prune's tests alone.
    MovedSpace {
        /// The root a frame was keyed by, or the instance a gesture was
        /// opened on, as the document held it.
        instance: SpokenNode,
        /// The root of the space it lives in now, as the document held
        /// it.
        into: SpokenNode,
    },
    /// The instance's geometry is FUSED into a drawn product together
    /// with other instances' (a boolean or placed union consumes
    /// both), so no display operation can address this instance
    /// separately — hiding or displacing it would have to hide or
    /// displace material that is not its. Refused typed rather than
    /// accepted-and-inert: an op that cannot take effect must say so
    /// (G3's honesty rule).
    FusedGeometry {
        /// The instance named, as the document held it.
        instance: SpokenNode,
        /// The placed body its geometry is fused into, as the document
        /// held it.
        body: SpokenNode,
        /// The other instances fused into the same body, as the
        /// document held them, in its order.
        others: Vec<SpokenNode>,
    },
}

impl core::fmt::Display for AdmissionFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoSuchNode { node } => {
                write!(f, "{node} is not in the document")
            }
            Self::NotAnInstance { node } => {
                write!(f, "{node} is not a part instance")
            }
            Self::InTheWorld { instance } => write!(
                f,
                "{instance} lives in the world's space: its placement pins it there (a world \
                 pin, or a mate onto a copy in the world), so the free-move probe refuses. \
                 Free-move moves a loose space, a copy nothing pins to the world"
            ),
            Self::NotEvaluated { instance } => write!(
                f,
                "{instance}'s space is not known yet: no evaluation of the current document \
                 has landed"
            ),
            Self::NoSpace { instance } => write!(
                f,
                "{instance} is in no space: its placement did not evaluate, so there is no \
                 picture of it to move"
            ),
            Self::MovedSpace { instance, into } if instance.id() == into.id() => write!(
                f,
                "{instance} left the loose space the free-move was drawn for and is a loose \
                 space of its own now, so that free-move is gone. A free-move of {instance} \
                 moves its own space"
            ),
            Self::MovedSpace { instance, into } => write!(
                f,
                "{instance} now lives in {into}'s loose space, not the one the free-move was \
                 drawn for, so that free-move is gone. A free-move of {instance} moves the \
                 space it lives in now"
            ),
            Self::FusedGeometry {
                instance,
                body,
                others,
            } => {
                let list: Vec<String> = others.iter().map(ToString::to_string).collect();
                write!(
                    f,
                    "{instance}'s geometry is fused into {body} together with {} — a display \
                     operation cannot address it separately",
                    list.join(", ")
                )
            }
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
            // Its sentence is that the node is not there: said from a
            // version that holds it again, it would contradict itself.
            unspoken @ Self::NoSuchNode { .. } => unspoken,
            Self::NotAnInstance { node } => Self::NotAnInstance { node: again(node) },
            Self::InTheWorld { instance } => Self::InTheWorld {
                instance: again(instance),
            },
            Self::NotEvaluated { instance } => Self::NotEvaluated {
                instance: again(instance),
            },
            Self::NoSpace { instance } => Self::NoSpace {
                instance: again(instance),
            },
            Self::MovedSpace { instance, into } => Self::MovedSpace {
                instance: again(instance),
                into: again(into),
            },
            Self::FusedGeometry {
                instance,
                body,
                others,
            } => Self::FusedGeometry {
                instance: again(instance),
                body: again(body),
                others: others.into_iter().map(again).collect(),
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
    /// An instance the document cannot admit a display operation on.
    ///
    /// The whole of [`AdmissionFault`], forwarded: every door here
    /// runs one of the admission tests, and the fault it raises is
    /// the test's own answer rather than a re-wording of it.
    Admission(AdmissionFault),
    /// The previewed frame is not a finite rigid motion (orthonormal
    /// linear part, det = +1 within [`RIGID_SLACK`]). Refused because a
    /// scaling or mirroring probe would draw geometry the document
    /// cannot mean, and because the pick path compares hit distances
    /// across instances, which only lengths-preserving frames keep
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
    /// A free-move operation named an instance that is not the one
    /// being probed — a preview or a commit for an instance other
    /// than the one the open probe was begun on.
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

/// **`node` is a live `InstantiatePart`**, or the typed refusal that
/// says which of the two ways it is not.
///
/// **The question is per-instance DISPLAY state**, not membership: hide
/// and free-move are keyed on the node with the identity a user hides
/// or probes, and only an `InstantiatePart` has one. A pattern node
/// draws several copies at once, so there is no single pose to probe
/// and no single body to hide.
///
/// It is NOT the mate MEMBER vocabulary, which
/// [`pncad::document::member_of`] owns and which additionally admits a
/// pattern copy — an `Instance(i)`-qualified head on a `Pattern` over a
/// live instance. The two differ because they ask different questions:
/// a copy is a member (the solve places it) while having no per-copy
/// display state of its own. An authoring door that gates on this
/// predicate refuses heads the solve already places.
///
/// **The two refusals are spelled apart** for
/// [`AdmissionFault::NoSuchNode`]'s reason: an absent id and a
/// wrong-kind node are different news at the doors that ANSWER a user's
/// operation, and a `bool` is a type that cannot carry the difference.
/// A caller for which both mean the same thing says so by discarding
/// the fault at its own call site, where the decision is readable,
/// rather than by being handed a door that discarded it first. The
/// properties panel is that caller, and
/// [`AdmissionFault::NoSuchNode`] carries why silence is right there.
///
/// # Errors
///
/// [`AdmissionFault::NoSuchNode`], [`AdmissionFault::NotAnInstance`].
pub fn instance_check(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Result<(), AdmissionFault> {
    match doc.node(node) {
        // Identity, not `session`'s "puts a part in" policy: the state
        // is keyed on this variant, and a new kind has none until G3
        // gives it some.
        Some(Node::InstantiatePart { .. }) => Ok(()),
        Some(_) => Err(AdmissionFault::NotAnInstance {
            node: doc.spoken(node),
        }),
        None => Err(AdmissionFault::NoSuchNode {
            node: doc.spoken(node),
        }),
    }
}

/// For each world placement, the instances whose geometry its copy
/// draws: the `InstantiatePart` nodes in its consuming-edge ancestry.
///
/// This is the map display state PROPAGATES through: the drawn scene
/// is keyed by placements, and a placement reads whatever body it
/// places — an instance, or a `Pattern` chain over one.
/// Display state names the INSTANCE (the thing with an identity a user
/// hides or probes); this map says which drawn copies that names.
fn instances_by_placement(
    doc: &Doc<ProfileProgram>,
) -> Vec<(RecipeNodeId, BTreeSet<RecipeNodeId>)> {
    crate::world::world_copies(doc)
        .into_iter()
        .map(|placement| {
            let instances = ancestry(doc, placement)
                .into_iter()
                .filter(|&id| instance_check(doc, id).is_ok())
                .collect();
            (placement, instances)
        })
        .collect()
}

/// Every node in `node`'s consuming-edge ancestry, `node` itself
/// included — "which nodes' work went into this drawn thing".
///
/// The one walk both consumers of that question run:
/// [`instances_by_placement`] filters it to instances (whose display
/// state propagates to the copies drawing them), and
/// [`placements_deriving_from`] inverts it. Two hand-written traversals of the same edges is how
/// they come to disagree about what an input is.
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
/// [`placements_deriving_from`]'s question asked of ONE pair, over the same
/// walk. What it is for: an entity minted at `source` and carried
/// upward reaches `node`'s output, so a node whose op leaves no trace
/// in a name — one that contributes no role segment — can still be
/// told which drawn entities passed through it.
pub fn derives_from(doc: &Doc<ProfileProgram>, node: RecipeNodeId, source: RecipeNodeId) -> bool {
    ancestry(doc, node).contains(&source)
}

/// **Every world placement whose copy derives from `node`** — the
/// placement itself when `node` is one, and every placement that
/// reaches it through consuming edges otherwise.
///
/// The inverse of [`ancestry`], and the answer to "if I am looking at
/// this recipe node, what in the picture is it responsible for". A
/// node that draws nothing on its own — a profile — has a non-empty
/// answer here, which is exactly the case that
/// makes the question worth asking: the profile IS the shape of the
/// walls the extrude above it drew.
///
/// Unlike [`drawn_targets`] this refuses nothing and excludes nothing:
/// a placement fusing several nodes' geometry is listed for each of
/// them.
/// The two differ because they are asked for different reasons —
/// `drawn_targets` backs an OPERATION that must address one node's
/// material alone, and this backs a HIGHLIGHT, where "several features
/// contributed to this body" is a true and useful thing to show.
pub fn placements_deriving_from(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
) -> BTreeSet<RecipeNodeId> {
    crate::world::world_copies(doc)
        .into_iter()
        .filter(|&placement| ancestry(doc, placement).contains(&node))
        .collect()
}

/// **The drawn copies a display operation on `instance` governs**, or
/// the typed refusal that says why none can be: every world placement
/// whose copy derives from the instance alone. A placement whose body
/// fuses this instance with others (a cross-instance boolean)
/// refuses [`AdmissionFault::FusedGeometry`] — the op could not take
/// effect without moving material that is not the instance's, and an
/// accepted-but-inert op is the dishonesty G3 forbids.
///
/// # Errors
///
/// [`AdmissionFault::NoSuchNode`], [`AdmissionFault::NotAnInstance`],
/// [`AdmissionFault::FusedGeometry`].
pub fn drawn_targets(
    doc: &Doc<ProfileProgram>,
    instance: RecipeNodeId,
) -> Result<BTreeSet<RecipeNodeId>, AdmissionFault> {
    // Absent and wrong-kind are two different answers, and the caller
    // that reports rather than refuses needs them apart.
    instance_check(doc, instance)?;
    let mut targets = BTreeSet::new();
    for (placement, instances) in instances_by_placement(doc) {
        if !instances.contains(&instance) {
            continue;
        }
        if instances.len() > 1 {
            return Err(AdmissionFault::FusedGeometry {
                instance: doc.spoken(instance),
                body: doc.spoken(crate::world::seat_of(doc, placement)),
                others: doc
                    .ids()
                    .iter()
                    .filter(|&&i| i != instance && instances.contains(&i))
                    .map(|&i| doc.spoken(i))
                    .collect(),
            });
        }
        targets.insert(placement);
    }
    Ok(targets)
}

/// The display admission test both operations share: a live instance
/// whose drawn geometry can be addressed separately.
///
/// # Errors
///
/// As [`drawn_targets`].
pub fn display_check(
    doc: &Doc<ProfileProgram>,
    instance: RecipeNodeId,
) -> Result<(), AdmissionFault> {
    drawn_targets(doc, instance).map(|_| ())
}

/// **The free-move admission test** (DI5): the loose space `instance`'s
/// copy lives in, by its root, read off `eval`.
///
/// A free-move moves the picture of a loose space: a copy whose bundle
/// pins less than its body needs, with everything pinned to it. So the
/// test asks the evaluation which space the copy lives in, not the
/// recipe which mates name it. Two copies mated to each other share a
/// space and move as one; a copy pinned to the world, or mated onto a
/// copy in the world, lives in the world's space and refuses
/// ([`AdmissionFault::InTheWorld`]).
///
/// [`display_check`] runs first, as for a hide, so an instance fused
/// into a product with another's refuses
/// ([`AdmissionFault::FusedGeometry`]) here too: the properties pane
/// offers the probe only where the hide is addressable, and the op and
/// the control it stands in for answer one test.
///
/// Spaces exist only after the solve, so the test reads them off an
/// evaluation (DI3): `eval` answers `doc`'s spaces, the session's
/// [`crate::session::DocSession::space_evaluation`], or is `None` when
/// none has landed that does
/// ([`AdmissionFault::NotEvaluated`]).
///
/// # Errors
///
/// [`AdmissionFault::NoSuchNode`], [`AdmissionFault::NotAnInstance`],
/// [`AdmissionFault::FusedGeometry`], [`AdmissionFault::NotEvaluated`],
/// [`AdmissionFault::NoSpace`], [`AdmissionFault::InTheWorld`].
pub fn free_move_check(
    doc: &Doc<ProfileProgram>,
    eval: Option<&Evaluation<f64>>,
    instance: RecipeNodeId,
) -> Result<RecipeNodeId, AdmissionFault> {
    display_check(doc, instance)?;
    let eval = eval.ok_or_else(|| AdmissionFault::NotEvaluated {
        instance: doc.spoken(instance),
    })?;
    loose_root(eval, instance).map_err(|world| match world {
        Unplaced::Refused => AdmissionFault::NoSpace {
            instance: doc.spoken(instance),
        },
        Unplaced::World => AdmissionFault::InTheWorld {
            instance: doc.spoken(instance),
        },
    })
}

/// **Whether `root` still names a loose space** — the test a held probe
/// is re-asked when the document moves ([`DisplayState::prune`]).
///
/// `eval` answers `doc`'s spaces, as for [`free_move_check`], or is
/// `None` when none has landed that does; then only what the document alone says is
/// asked (the root is gone), and the space question waits for the
/// landing, which prunes again.
///
/// # Errors
///
/// [`AdmissionFault::NoSuchNode`], and with an evaluation
/// [`AdmissionFault::NoSpace`], [`AdmissionFault::InTheWorld`] and
/// [`AdmissionFault::MovedSpace`].
pub fn space_check(
    doc: &Doc<ProfileProgram>,
    eval: Option<&Evaluation<f64>>,
    root: RecipeNodeId,
) -> Result<(), AdmissionFault> {
    frame_check(doc, eval, root, Answers::All)
}

/// [`space_check`], over which answers it acts on: the frame test of
/// [`DisplayState::prune`] and [`DisplayState::prune_structural`].
fn frame_check(
    doc: &Doc<ProfileProgram>,
    eval: Option<&Evaluation<f64>>,
    root: RecipeNodeId,
    answers: Answers,
) -> Result<(), AdmissionFault> {
    if doc.node(root).is_none() {
        return Err(AdmissionFault::NoSuchNode {
            node: doc.spoken(root),
        });
    }
    eval.map_or(Ok(()), |eval| in_space(doc, eval, root, root, answers))
}

/// **Whether a held gesture still moves the space its instance lives
/// in** — the in-flight half of [`DisplayState::prune`]. The gesture
/// names its instance and moves its root's space, so both are asked:
/// the instance is still in the document, and, with an evaluation, it
/// still lives in that space. A frame keyed by a root is asked about
/// the root alone ([`space_check`]); a gesture opened on another copy
/// of the space would otherwise go on steering the space after its own
/// copy was deleted or split off.
///
/// The root is asked too, whether it is still in the document. A delete
/// that takes the root takes the copy as well, because every copy in a
/// space reads its root. An undo need not: a root inserted after the
/// copy, and then mated onto by it, is undone mate first and insertion
/// after, which leaves the copy, its own space now, with no root, while
/// the last landing still answers the older document.
fn gesture_check(
    doc: &Doc<ProfileProgram>,
    eval: Option<&Evaluation<f64>>,
    probe: Probe,
    answers: Answers,
) -> Result<(), AdmissionFault> {
    let Probe { root, instance } = probe;
    if doc.node(instance).is_none() {
        return Err(AdmissionFault::NoSuchNode {
            node: doc.spoken(instance),
        });
    }
    space_check(doc, None, root)?;
    eval.map_or(Ok(()), |eval| in_space(doc, eval, instance, root, answers))
}

/// **Which of an evaluation's space answers a prune acts on.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Answers {
    /// Every one: a landing of the committed document.
    All,
    /// Every one but a refused placement: a landing of a value
    /// gesture's scratch, which shares the committed graph but not its
    /// values ([`DisplayState::prune_structural`]). A pin, a join and a
    /// split are the graph's, so the scratch answers them for the
    /// committed document too. A refusal may be a value's, and the
    /// evaluation does not say which decided it, so every refusal waits
    /// for a committed landing — a graph-decided one too (two mates
    /// that over-constrain a copy), which is then said at the drag's
    /// end rather than mid-drag.
    Structural,
}

/// **Whether `asked`'s copy lives in the loose space `root` roots**, by
/// `eval`: the one mapping from a space answer to a fault, for a frame
/// (asked about its root) and a gesture (asked about its own copy).
fn in_space(
    doc: &Doc<ProfileProgram>,
    eval: &Evaluation<f64>,
    asked: RecipeNodeId,
    root: RecipeNodeId,
    answers: Answers,
) -> Result<(), AdmissionFault> {
    match loose_root(eval, asked) {
        Ok(found) if found == root => Ok(()),
        Ok(found) => Err(AdmissionFault::MovedSpace {
            instance: doc.spoken(asked),
            into: doc.spoken(found),
        }),
        Err(Unplaced::World) => Err(AdmissionFault::InTheWorld {
            instance: doc.spoken(asked),
        }),
        Err(Unplaced::Refused) if answers == Answers::Structural => Ok(()),
        Err(Unplaced::Refused) => Err(AdmissionFault::NoSpace {
            instance: doc.spoken(asked),
        }),
    }
}

/// Why a node's value is in no loose space.
enum Unplaced {
    /// It lives in the world's space.
    World,
    /// The evaluation gave it no value, so it lives in no space.
    Refused,
}

/// The root of the loose space `node`'s value lives in.
///
/// [`Evaluation::space`] reads a node absent from its spaces as the
/// world's, which is also how it reads a refused placement, so the
/// value is asked first: a node that did not evaluate is in no space.
fn loose_root(eval: &Evaluation<f64>, node: RecipeNodeId) -> Result<RecipeNodeId, Unplaced> {
    if eval.value(node).is_none() {
        return Err(Unplaced::Refused);
    }
    match eval.space(node) {
        Space::Loose { root } => Ok(root),
        Space::World => Err(Unplaced::World),
    }
}

/// Whether `frame` is a finite rigid motion: finite everywhere,
/// orthonormal columns, determinant +1 — each within [`RIGID_SLACK`].
fn is_rigid(frame: &Frame) -> bool {
    if !frame.is_finite() {
        return false;
    }
    let [c0, c1, c2] = frame.columns.map(Vec3::from_array);
    let unit = |c: Vec3<f64>| (c.dot(c) - 1.0).abs() <= RIGID_SLACK;
    let perp = |a: Vec3<f64>, b: Vec3<f64>| a.dot(b).abs() <= RIGID_SLACK;
    unit(c0)
        && unit(c1)
        && unit(c2)
        && perp(c0, c1)
        && perp(c1, c2)
        && perp(c0, c2)
        && (frame.determinant() - 1.0).abs() <= RIGID_SLACK
}

/// **The probe's words for the three G1 states**, declared once and
/// handed to [`g1::Slot`] at every door.
///
/// The machine is shared with the value drag and the vocabularies are
/// not: these three sentences are about an instance's probe, and the
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
/// as a value, in BOTH keyings — the raw instance-keyed facts (what
/// the chrome's checkboxes and panels show) and their resolution onto
/// the WORLD PLACEMENTS the scene is actually drawn under. A hide
/// reaches them through [`drawn_targets`] (a patterned instance's
/// geometry is emitted under the placements of the pattern's copies),
/// and a free-move frame reaches every placement in its loose space.
///
/// Owned rather than borrowed so a caller can hold one across the
/// mutation that would invalidate a borrow, and because the maps are
/// a handful of entries.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayView {
    /// Instances the picture drops (instance-keyed; the chrome's
    /// reading).
    pub hidden: BTreeSet<RecipeNodeId>,
    /// Loose space's root → the display frame composed over its drawn
    /// copies. An in-flight preview overrides that space's committed
    /// value.
    pub moved: BTreeMap<RecipeNodeId, Frame>,
    /// The placements whose copies the hidden instances govern — what
    /// the scene and the pick index drop.
    pub hidden_placements: BTreeSet<RecipeNodeId>,
    /// Placement → the probe frame governing its copy — what the scene
    /// displaces and marks, and the pick carries rays into.
    pub moved_placements: BTreeMap<RecipeNodeId, Frame>,
}

impl DisplayView {
    /// The view that hides nothing and moves nothing — what every
    /// non-assembly consumer reads.
    pub fn none() -> Self {
        Self::default()
    }
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
    /// The node the display fact was keyed on: the instance, for a
    /// hide and a killed gesture, and the loose space's root — a
    /// placement, which need not be an instance — for a discarded
    /// frame.
    pub instance: RecipeNodeId,
    /// Why the document no longer admits it.
    ///
    /// **[`AdmissionFault`] and not [`DisplayFault`]**, because the
    /// causes a prune can withdraw on are exactly what its tests
    /// ([`display_check`], [`space_check`] and the gesture's, which
    /// asks its own copy the same space question) answer — and because
    /// [`crate::frame::Withdrawal`] joins several of these into one
    /// sentence with [`crate::frame::LIST_SEPARATOR`], which is a
    /// claim about what those sentences can say. A field typed as the
    /// whole vocabulary would have left that claim resting on which
    /// faults `prune`'s two callees happen to raise.
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
///   for the instance not to be drawn, and the document did not answer
///   that question differently; it made the question unaskable — the
///   instance is gone, or its geometry is fused into a product that
///   cannot be addressed without hiding material that is not its. The
///   picture gains geometry the user had taken out of it, or loses the
///   instance entirely, and nothing takes the hide's place.
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
    /// Loose spaces whose COMMITTED free-move frame was discarded, each
    /// by its root, with the [`space_check`] fault that discarded it — a
    /// pin to the world landing, the space's placement refused, the
    /// root's copy joining another space, or the root being gone.
    ///
    /// **Only committed probes.** A gesture in flight when the
    /// transition lands dies too; it is `killed_gesture`, because it
    /// is not a supersession.
    pub superseded: Vec<Withdrawn>,
    /// Instances whose HIDE was dropped, with the [`display_check`]
    /// fault that dropped it. The instance is drawn again where the
    /// picture can no longer leave it out, and is gone where the
    /// document no longer holds it — which of the two is what the
    /// fault says.
    pub dropped_hides: Vec<Withdrawn>,
    /// The in-flight free-move gesture this prune killed, by the
    /// instance it was opened on, with the fault that killed it — the
    /// instance gone, or no longer in the space the gesture moves — or
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

/// **What a free-move gesture holds**: the loose space it moves, by its
/// root, and the instance the gesture was opened on, which the preview
/// and the commit name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Probe {
    root: RecipeNodeId,
    instance: RecipeNodeId,
}

/// **The one home** for hide and free-move state (the seam-friction
/// inventory discipline: no per-widget shadows). Owned by the session;
/// every mutation is a typed operation routed through it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayState {
    hidden: BTreeSet<RecipeNodeId>,
    /// The committed free-move frames, keyed by the root of the loose
    /// space each was drawn for (DI5).
    moves: BTreeMap<RecipeNodeId, Frame>,
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

    /// Whether `instance` is hidden.
    pub fn is_hidden(&self, instance: RecipeNodeId) -> bool {
        self.hidden.contains(&instance)
    }

    /// The hidden set.
    pub fn hidden(&self) -> &BTreeSet<RecipeNodeId> {
        &self.hidden
    }

    /// The committed free-move frame of the loose space rooted at
    /// `root`, if any. The in-flight preview is NOT reported here —
    /// [`DisplayState::view`] is the door that composes both.
    pub fn free_move_of(&self, root: RecipeNodeId) -> Option<&Frame> {
        self.moves.get(&root)
    }

    /// The instance a free-move gesture is currently probing, if one
    /// is in flight.
    pub fn probing(&self) -> Option<RecipeNodeId> {
        self.free_move.held().map(|probe| probe.instance)
    }

    /// The snapshot the scene and pick paths consume, resolved onto
    /// `doc`'s placements: a hide through [`drawn_targets`], a free-move
    /// frame over every placement whose copy lives in the frame's loose
    /// space, read off `eval`, the session's landed evaluation.
    ///
    /// Resolution failures are skipped rather than surfaced here: the
    /// operations that write this state run the same check and refuse
    /// typed, and [`DisplayState::prune`] discards entries the
    /// document has since made illegal. Between an edit and the
    /// evaluation answering it, `eval` answers the document before the
    /// edit, so a frame is drawn over its space's last known copies:
    /// the display-only window, which the landing's prune closes.
    pub fn view(&self, doc: &Doc<ProfileProgram>, eval: Option<&Evaluation<f64>>) -> DisplayView {
        let mut moved = self.moves.clone();
        if let Some((probe, &frame)) = self.free_move.previewing() {
            moved.insert(probe.root, frame);
        }
        let mut hidden_placements = BTreeSet::new();
        for &instance in &self.hidden {
            if let Ok(targets) = drawn_targets(doc, instance) {
                hidden_placements.extend(targets);
            }
        }
        let mut moved_placements = BTreeMap::new();
        if let Some(eval) = eval {
            for placement in doc.placements() {
                if let Ok(root) = loose_root(eval, placement)
                    && let Some(&frame) = moved.get(&root)
                {
                    moved_placements.insert(placement, frame);
                }
            }
        }
        DisplayView {
            hidden: self.hidden.clone(),
            moved,
            hidden_placements,
            moved_placements,
        }
    }

    /// Hide or show one instance.
    ///
    /// # Errors
    ///
    /// [`AdmissionFault::NoSuchNode`] for an id the document does not
    /// hold, [`AdmissionFault::NotAnInstance`] — hiding is a
    /// per-instance operation; other node kinds draw through their own
    /// placements and have no instance identity to hide by — and
    /// [`AdmissionFault::FusedGeometry`] for an instance the drawn
    /// picture cannot address separately.
    pub fn set_hidden(
        &mut self,
        doc: &Doc<ProfileProgram>,
        instance: RecipeNodeId,
        hidden: bool,
    ) -> Result<(), DisplayFault> {
        display_check(doc, instance)?;
        let changed = if hidden {
            self.hidden.insert(instance)
        } else {
            self.hidden.remove(&instance)
        };
        if changed {
            self.revision += 1;
        }
        Ok(())
    }

    /// Open a free-move gesture on the loose space `instance`'s copy
    /// lives in (DI5), read off `eval` as [`free_move_check`] reads it.
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
        instance: RecipeNodeId,
    ) -> Result<(), DisplayFault> {
        self.free_move.begin(free_move_words(), || {
            let root = free_move_check(doc, eval, instance)?;
            Ok(Probe { root, instance })
        })
    }

    /// Stream one preview frame into the in-flight gesture, replacing
    /// the last: the composed display value is `frame`, never an
    /// accumulation of deltas. The replacement and the two refusals are
    /// [`g1::Slot::preview`]'s; what is this door's own is the
    /// rigid-motion check.
    ///
    /// **It names the instance it is probing**, and the name is
    /// checked before the frame is: a drag on a second instance's
    /// field cannot compose its frame onto the instance an open probe
    /// holds. [`DisplayFault::WrongFreeMove`] carries the argument.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`], [`DisplayFault::WrongFreeMove`],
    /// [`DisplayFault::NonRigidFrame`].
    pub fn preview_free_move(
        &mut self,
        instance: RecipeNodeId,
        frame: Frame,
    ) -> Result<(), DisplayFault> {
        self.free_move.preview(
            free_move_words(),
            |probed| probed.instance == instance,
            |_| {
                if is_rigid(&frame) {
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
    /// frame of the loose space it moves, keyed by the space's root. It
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
    /// Names its instance for [`DisplayState::preview_free_move`]'s
    /// reason.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`], [`DisplayFault::WrongFreeMove`].
    pub fn commit_free_move(&mut self, instance: RecipeNodeId) -> Result<(), DisplayFault> {
        let landed = self
            .free_move
            .commit(free_move_words(), |probed| probed.instance == instance)?;
        if let Some((probed, frame)) = landed {
            if frame.is_identity_bits() {
                self.moves.remove(&probed.root);
            } else {
                self.moves.insert(probed.root, frame);
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
    /// frame whose root no longer names a loose space (a placement pins
    /// the copy, it joins another space, its placement is refused, or
    /// the root is gone; DI5),
    /// drop hidden entries whose instance the picture can no longer
    /// address, and kill an in-flight gesture whose instance is gone or
    /// no longer lives in the space it moves.
    ///
    /// `eval` is an evaluation of `doc` itself, or `None`. Spaces exist
    /// only after the solve, so the session prunes twice: when the
    /// document changes, with no evaluation, which withdraws what the
    /// document alone says is gone, and when the evaluation answering it
    /// lands, which withdraws a space a placement now pins, joins or
    /// refuses ([`space_check`]). A landing of a value gesture's scratch
    /// prunes through [`DisplayState::prune_structural`] instead.
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
    /// decides — a pin, a join, a split, a node gone — and not on a
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
            .retain(|&root, _| match frame_check(doc, eval, root, answers) {
                Ok(()) => true,
                Err(cause) => {
                    superseded.push(Withdrawn {
                        instance: root,
                        cause,
                    });
                    false
                }
            });
        let dropped_hides: Vec<Withdrawn> = self
            .hidden
            .iter()
            .filter_map(|&instance| {
                display_check(doc, instance)
                    .err()
                    .map(|cause| Withdrawn { instance, cause })
            })
            .collect();
        for dropped in &dropped_hides {
            self.hidden.remove(&dropped.instance);
        }
        // The fault is CARRIED, not tested: `is_err()` here would
        // throw away the one value that says why the drag under the
        // user's hand stopped, at the instant it is computed.
        let killed_gesture = self.free_move.held().and_then(|probe| {
            gesture_check(doc, eval, *probe, answers)
                .err()
                .map(|cause| Withdrawn {
                    instance: probe.instance,
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
    /// asked for exactly this. A per-instance notice would report the
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
    /// wherever the incoming document happens to hold a loose space at
    /// that root — a report that says nothing was withdrawn
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
            hidden,
            moves,
            free_move,
            revision,
        } = self;
        if !hidden.is_empty() || !moves.is_empty() || free_move.held().is_some() {
            *revision += 1;
        }
        hidden.clear();
        moves.clear();
        free_move.discard();
    }
}
