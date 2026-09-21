//! Layer-3 display state for assemblies: per-instance **hide** and the
//! **free-move** fit probe (GUI-DESIGN G3), each with exactly one home.
//!
//! # What this state is, and what it is not
//!
//! Both facts here are about the PICTURE, never about the model. A
//! hidden instance stays in the document and in the feature tree; only
//! the drawn scene and the pick index drop it. A free-move transform is
//! the G3 fit probe — a display frame composed over an instance's drawn
//! placement so a user can hold a part against another BEFORE authoring
//! the mate — and it involves no solver, enters no history, and is
//! never persisted: `save` writes the document, and a reopened session
//! starts with this state empty. That is not an accident of the code
//! path; it is G3's ratified boundary, and the round-trip row pins it.
//!
//! # Eligibility is derived from the document
//!
//! Free-move accepts only a **completely-unconstrained** instance: one
//! that appears in NO mate node ([`mates_naming`] scans the recipe's
//! own `Node::Mate` references — the authored data, not the solver's
//! state). An instance any mate names refuses typed
//! ([`AdmissionFault::MateConstrained`], listing the mates), because its
//! pose is mate-derived and a display value contradicting it would
//! draw a relation the document does not have.
//!
//! # Display state names instances; the picture is drawn under roots
//!
//! The scene and the pick index emit geometry per PRODUCT ROOT, and an
//! instance a `Pattern` or `Transform` consumes is not a root — the
//! node above it is. Display state therefore PROPAGATES: an operation
//! names the instance, and [`drawn_targets`] resolves it to every root
//! whose geometry derives from that instance alone (hiding a patterned
//! instance hides all its placed copies; probing it displaces them
//! under one frame — the pattern replicates the instance, and the
//! display fact is the instance's). A root that fuses SEVERAL
//! instances' geometry (a cross-instance boolean) can be addressed by
//! none of them separately, and the op refuses typed
//! ([`AdmissionFault::FusedGeometry`]) — the alternative, accepting the
//! op and drawing nothing different, is the silent no-op G3's honesty
//! rule forbids.
//!
//! # Supersession: the probe dies when the mate lands
//!
//! When an edit makes a free-moved instance mate-constrained, its
//! free-move value is **discarded** — removed outright, not zeroed and
//! kept — by [`DisplayState::prune`], which the session runs against
//! every new document. Discard rather than zero because the value's
//! LEGALITY is derived from the document: an entry for a constrained
//! instance is unrepresentable state, not a transform that happens to
//! be identity. The instance is thereafter drawn at its solved
//! placement, undistinguished, which is exactly the honesty rule: the
//! probe treatment marks "this is not where the document puts it", and
//! after the mate lands the document DOES put it there.
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
//! they can only DISCARD a probe by making its instance constrained.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::{BTreeMap, BTreeSet};

use pncad::document::{Doc, Frame, Node, ProfileProgram, RecipeNodeId};

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
/// # Each arm names its subject in the strongest vocabulary true of it
///
/// The rule over the whole enum, not a property of one arm: an arm
/// whose subject IS a part instance says **"instance N"**
/// ([`AdmissionFault::MateConstrained`], [`AdmissionFault::FusedGeometry`])
/// — the word the properties panel and the feature tree use for the
/// thing a user hides or probes. An arm whose whole content is that
/// the id does NOT denote one says **"node N"**
/// ([`AdmissionFault::NoSuchNode`], [`AdmissionFault::NotAnInstance`]),
/// because calling it an instance there would assert the very thing
/// the arm is denying.
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
/// are these four and a fifth cannot arrive without an arm here. The
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
    /// same frame, and that type's ratified rule is that a vanished
    /// reference is rendered there *while the affordances that need a
    /// live entity switch off*. The section switching off IS the second
    /// clause; a sentence at the affordance would be one fact spelled
    /// twice in one pane.
    NoSuchNode {
        /// The id named.
        node: RecipeNodeId,
    },
    /// The node is not an `InstantiatePart`, so it has no per-instance
    /// display state to set.
    NotAnInstance {
        /// The node named.
        node: RecipeNodeId,
    },
    /// The instance participates in a mate, so its pose is
    /// mate-derived and the free-move probe refuses (G3: the probe is
    /// for completely-unconstrained instances only).
    MateConstrained {
        /// The instance.
        instance: RecipeNodeId,
        /// Every mate node naming it, document order.
        mates: Vec<RecipeNodeId>,
    },
    /// The instance's geometry is FUSED into a drawn product together
    /// with other instances' (a boolean or placed union consumes
    /// both), so no display operation can address this instance
    /// separately — hiding or displacing it would have to hide or
    /// displace material that is not its. Refused typed rather than
    /// accepted-and-inert: an op that cannot take effect must say so
    /// (G3's honesty rule).
    FusedGeometry {
        /// The instance named.
        instance: RecipeNodeId,
        /// The drawn root its geometry is fused into.
        root: RecipeNodeId,
        /// The other instances fused into the same root.
        others: Vec<RecipeNodeId>,
    },
}

impl core::fmt::Display for AdmissionFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoSuchNode { node } => {
                write!(f, "node {} is not in the document", node.0)
            }
            Self::NotAnInstance { node } => {
                write!(f, "node {} is not a part instance", node.0)
            }
            Self::MateConstrained { instance, mates } => {
                let list: Vec<String> = mates.iter().map(|m| m.0.to_string()).collect();
                write!(
                    f,
                    "instance {} is mate-constrained (mate node(s) {}): its pose is \
                     mate-derived, so the free-move probe refuses — delete the mate(s) if \
                     free relative motion is intended",
                    instance.0,
                    list.join(", ")
                )
            }
            Self::FusedGeometry {
                instance,
                root,
                others,
            } => {
                let list: Vec<String> = others.iter().map(|o| o.0.to_string()).collect();
                write!(
                    f,
                    "instance {}'s geometry is fused into node {} together with instance(s) {} — \
                     a display operation cannot address it separately",
                    instance.0,
                    root.0,
                    list.join(", ")
                )
            }
        }
    }
}

impl core::error::Error for AdmissionFault {}

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
/// [`DisplayFault::Admission`] carries the four faults that say an
/// INSTANCE cannot hold display state — the answers of the admission
/// tests [`display_check`] and [`free_move_check`], each naming the id
/// it is about. The four arms beside it are about a gesture or a frame
/// and name no id at all.
///
/// The families are apart because one of them is joined into a list
/// and the other is not: [`DisplayState::prune`] fills every
/// [`Withdrawn`] from the two admission tests, so a withdrawal's
/// cause is an [`AdmissionFault`] by signature rather than by a
/// property of those two functions' error sets, and
/// [`AdmissionFault`]'s own docs carry what the join then needs of
/// the four sentences.
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

/// Every mate node naming `instance` on either side, document order —
/// the completely-unconstrained test's evidence, read from the
/// AUTHORED references (a dangling or misdirected reference still
/// names its head node; participation is about what the document
/// says, not about what currently resolves).
pub fn mates_naming(doc: &Doc<ProfileProgram>, instance: RecipeNodeId) -> Vec<RecipeNodeId> {
    doc.order()
        .iter()
        .copied()
        .filter(|&id| match doc.node(id) {
            Some(Node::Mate { a, b, .. }) => a.name.node == instance || b.name.node == instance,
            _ => false,
        })
        .collect()
}

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
        Some(Node::InstantiatePart { .. }) => Ok(()),
        Some(_) => Err(AdmissionFault::NotAnInstance { node }),
        None => Err(AdmissionFault::NoSuchNode { node }),
    }
}

/// For each product root, the instances whose geometry it draws: the
/// `InstantiatePart` nodes in its consuming-edge ancestry (the root
/// itself included).
///
/// This is the map display state PROPAGATES through: the drawn scene
/// is keyed by product roots, and an instance a `Pattern` (or a
/// `Transform` chain) consumes is not a root — the node above it is.
/// Display state names the INSTANCE (the thing with an identity a user
/// hides or probes); this map says which drawn roots that names.
fn instances_by_root(doc: &Doc<ProfileProgram>) -> Vec<(RecipeNodeId, BTreeSet<RecipeNodeId>)> {
    doc.roots()
        .iter()
        .map(|&root| {
            let instances = ancestry(doc, root)
                .into_iter()
                .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
                .collect();
            (root, instances)
        })
        .collect()
}

/// Every node in `root`'s consuming-edge ancestry, `root` itself
/// included — "which nodes' work went into this drawn thing".
///
/// The one walk both consumers of that question run:
/// [`instances_by_root`] filters it to instances (whose display state
/// propagates to the roots drawing them), and [`roots_deriving_from`]
/// inverts it. Two hand-written traversals of the same edges is how
/// they come to disagree about what an input is.
fn ancestry(doc: &Doc<ProfileProgram>, root: RecipeNodeId) -> BTreeSet<RecipeNodeId> {
    let mut seen = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if !seen.insert(id) {
            continue;
        }
        if let Some(node) = doc.node(id) {
            stack.extend(node.inputs());
        }
    }
    seen
}

/// **Whether `node`'s geometry derives from `source`** — `source` in
/// `node`'s consuming-edge ancestry, `node` itself included.
///
/// [`roots_deriving_from`]'s question asked of ONE pair, over the same
/// walk. What it is for: an entity minted at `source` and carried
/// upward reaches `node`'s output, so a node whose op leaves no trace
/// in a name — a `Transform`, which contributes no role segment by
/// construction — can still be told which drawn entities passed
/// through it.
pub fn derives_from(doc: &Doc<ProfileProgram>, node: RecipeNodeId, source: RecipeNodeId) -> bool {
    ancestry(doc, node).contains(&source)
}

/// **Every product root whose geometry derives from `node`** — the
/// root itself when `node` is one, and every root that reaches it
/// through consuming edges otherwise.
///
/// The inverse of [`ancestry`], and the answer to "if I am looking at
/// this recipe node, what in the picture is it responsible for". A
/// node that draws nothing on its own — a profile, a datum, a sketch
/// plane — has a non-empty answer here, which is exactly the case that
/// makes the question worth asking: the profile IS the shape of the
/// walls the extrude above it drew.
///
/// Unlike [`drawn_targets`] this refuses nothing and excludes nothing:
/// a root fusing several nodes' geometry is listed for each of them.
/// The two differ because they are asked for different reasons —
/// `drawn_targets` backs an OPERATION that must address one node's
/// material alone, and this backs a HIGHLIGHT, where "several features
/// contributed to this body" is a true and useful thing to show.
pub fn roots_deriving_from(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
) -> BTreeSet<RecipeNodeId> {
    doc.roots()
        .iter()
        .copied()
        .filter(|&root| ancestry(doc, root).contains(&node))
        .collect()
}

/// **The drawn roots a display operation on `instance` governs**, or
/// the typed refusal that says why none can be: every product root
/// whose geometry derives from the instance alone. A root whose
/// geometry fuses this instance with others (a cross-instance boolean)
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
    for (root, instances) in instances_by_root(doc) {
        if !instances.contains(&instance) {
            continue;
        }
        if instances.len() > 1 {
            return Err(AdmissionFault::FusedGeometry {
                instance,
                root,
                others: instances.into_iter().filter(|&i| i != instance).collect(),
            });
        }
        targets.insert(root);
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

/// The free-move admission test: [`display_check`] plus no mate names
/// the instance.
///
/// # Errors
///
/// [`AdmissionFault::NoSuchNode`], [`AdmissionFault::NotAnInstance`],
/// [`AdmissionFault::FusedGeometry`], [`AdmissionFault::MateConstrained`].
pub fn free_move_check(
    doc: &Doc<ProfileProgram>,
    instance: RecipeNodeId,
) -> Result<(), AdmissionFault> {
    display_check(doc, instance)?;
    let mates = mates_naming(doc, instance);
    if mates.is_empty() {
        Ok(())
    } else {
        Err(AdmissionFault::MateConstrained { instance, mates })
    }
}

/// Whether `frame` is a finite rigid motion: finite everywhere,
/// orthonormal columns, determinant +1 — each within [`RIGID_SLACK`].
fn is_rigid(frame: &Frame) -> bool {
    if !frame.is_finite() {
        return false;
    }
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let [c0, c1, c2] = frame.columns;
    let unit = |c: [f64; 3]| (dot(c, c) - 1.0).abs() <= RIGID_SLACK;
    let perp = |a: [f64; 3], b: [f64; 3]| dot(a, b).abs() <= RIGID_SLACK;
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
/// the PRODUCT ROOTS the scene is actually drawn under
/// ([`drawn_targets`] — a patterned instance's geometry is emitted
/// under the `Pattern` root, and this resolution is what makes hide
/// and free-move reach it there).
///
/// Owned rather than borrowed so a caller can hold one across the
/// mutation that would invalidate a borrow, and because the maps are
/// a handful of entries.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayView {
    /// Instances the picture drops (instance-keyed; the chrome's
    /// reading).
    pub hidden: BTreeSet<RecipeNodeId>,
    /// Instance → the display frame composed over its drawn placement.
    /// An in-flight preview overrides that instance's committed value.
    pub moved: BTreeMap<RecipeNodeId, Frame>,
    /// The drawn roots the hidden instances govern — what the scene
    /// and the pick index drop.
    pub hidden_roots: BTreeSet<RecipeNodeId>,
    /// Drawn root → the probe frame governing it — what the scene
    /// displaces and marks, and the pick carries rays into.
    pub moved_roots: BTreeMap<RecipeNodeId, Frame>,
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
    /// The instance the display fact was keyed on.
    pub instance: RecipeNodeId,
    /// Why the document no longer admits it.
    ///
    /// **[`AdmissionFault`] and not [`DisplayFault`]**, because the
    /// causes a prune can withdraw on are exactly what the two
    /// admission tests answer — and because
    /// [`crate::frame::Withdrawal`] joins several of these into one
    /// sentence with [`crate::frame::LIST_SEPARATOR`], which is a
    /// claim about what the four can say. A field typed as the whole
    /// vocabulary would have left that claim resting on which faults
    /// `prune`'s two callees happen to raise.
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
///   was an answer to "where does this part go", and the mate that
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
    /// Instances whose COMMITTED free-move probe was discarded, with
    /// the [`free_move_check`] fault that discarded it — a mate
    /// landing, a fuse, or the instance being gone.
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
    /// The in-flight free-move gesture this prune killed, with the
    /// [`free_move_check`] fault that killed it — or `None`, which is
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
    /// Private: [`DisplayState::prune`] is its one caller, deciding
    /// whether the revision moved. The two types themselves are `pub`
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
    fn is_empty(&self) -> bool {
        let Self {
            superseded,
            dropped_hides,
            killed_gesture,
        } = self;
        superseded.is_empty() && dropped_hides.is_empty() && killed_gesture.is_none()
    }
}

/// **The one home** for hide and free-move state (the seam-friction
/// inventory discipline: no per-widget shadows). Owned by the session;
/// every mutation is a typed operation routed through it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayState {
    hidden: BTreeSet<RecipeNodeId>,
    moves: BTreeMap<RecipeNodeId, Frame>,
    /// The free-move gesture in flight, if any.
    ///
    /// **Not spelled `gesture`.** [`crate::session::DocSession`] holds
    /// a `gesture` of its own — the VALUE drag a slot or parameter
    /// field opens — and the two are independent drags that can be
    /// open at once, so one field name must not stand for both. Why
    /// that overlap is sound, the identity it rests on and the
    /// ratified change (DI5) that ends the argument are stated at
    /// [`crate::session::SessionOp::permitted_during_value_gesture`].
    free_move: g1::Slot<RecipeNodeId, Frame>,
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

    /// The committed free-move frame of `instance`, if any. The
    /// in-flight preview is NOT reported here — [`DisplayState::view`]
    /// is the door that composes both.
    pub fn free_move_of(&self, instance: RecipeNodeId) -> Option<&Frame> {
        self.moves.get(&instance)
    }

    /// The instance a free-move gesture is currently probing, if one
    /// is in flight.
    pub fn probing(&self) -> Option<RecipeNodeId> {
        self.free_move.held().copied()
    }

    /// The snapshot the scene and pick paths consume, resolved onto
    /// `doc`'s product roots ([`drawn_targets`]).
    ///
    /// Resolution failures are skipped rather than surfaced here: the
    /// operations that write this state run the same check and refuse
    /// typed, and [`DisplayState::prune`] discards entries the
    /// document has since made illegal — so a skip is only reachable
    /// in the one-frame window between an edit and its prune.
    pub fn view(&self, doc: &Doc<ProfileProgram>) -> DisplayView {
        let mut moved = self.moves.clone();
        if let Some((&instance, &frame)) = self.free_move.previewing() {
            moved.insert(instance, frame);
        }
        let mut hidden_roots = BTreeSet::new();
        for &instance in &self.hidden {
            if let Ok(targets) = drawn_targets(doc, instance) {
                hidden_roots.extend(targets);
            }
        }
        let mut moved_roots = BTreeMap::new();
        for (&instance, &frame) in &moved {
            if let Ok(targets) = drawn_targets(doc, instance) {
                for root in targets {
                    moved_roots.insert(root, frame);
                }
            }
        }
        DisplayView {
            hidden: self.hidden.clone(),
            moved,
            hidden_roots,
            moved_roots,
        }
    }

    /// Hide or show one instance.
    ///
    /// # Errors
    ///
    /// [`AdmissionFault::NoSuchNode`] for an id the document does not
    /// hold, [`AdmissionFault::NotAnInstance`] — hiding is a
    /// per-instance operation; other node kinds draw through their own
    /// roots and have no instance identity to hide by — and
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

    /// Open a free-move gesture on a completely-unconstrained
    /// instance.
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
        instance: RecipeNodeId,
    ) -> Result<(), DisplayFault> {
        self.free_move.begin(free_move_words(), || {
            free_move_check(doc, instance)?;
            Ok(instance)
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
            |probed| *probed == instance,
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

    /// Land the gesture: its last previewed frame becomes the
    /// instance's committed probe value. The no-move rule — a gesture
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
            .commit(free_move_words(), |probed| *probed == instance)?;
        if let Some((probed, frame)) = landed {
            if frame.is_identity_bits() {
                self.moves.remove(&probed);
            } else {
                self.moves.insert(probed, frame);
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

    /// Reconcile this state with a new document: DISCARD every probe
    /// whose instance is now mate-constrained, fused, or gone (the
    /// supersession rule, module docs), drop hidden entries whose
    /// instance the picture can no longer address, and kill an
    /// in-flight gesture whose instance became ineligible.
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
    pub fn prune(&mut self, doc: &Doc<ProfileProgram>) -> PruneReport {
        let mut superseded = Vec::new();
        self.moves
            .retain(|&instance, _| match free_move_check(doc, instance) {
                Ok(()) => true,
                Err(cause) => {
                    superseded.push(Withdrawn { instance, cause });
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
        let killed_gesture = self.free_move.held().and_then(|&instance| {
            free_move_check(doc, instance)
                .err()
                .map(|cause| Withdrawn { instance, cause })
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
    /// nodes. Asking [`free_move_check`] about them would answer
    /// `Ok(())` wherever the incoming document happens to hold a free
    /// instance at that id — a report that says nothing was withdrawn
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
