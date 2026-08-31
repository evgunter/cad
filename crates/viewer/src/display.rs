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
//! ([`DisplayFault::MateConstrained`], listing the mates), because its
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
//! ([`DisplayFault::FusedGeometry`]) — the alternative, accepting the
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
//! # Which history holds a committed free-move: none
//!
//! The G1 preview/commit shape applies — a gesture streams preview
//! frames and lands exactly one committed value — but the commit
//! replaces this state's entry and enters NO history: the plan's undo
//! note governs document state only, and no layer-3 history exists in
//! v1. Undo/redo therefore never change what is hidden or probed;
//! they can only DISCARD a probe by making its instance constrained.

use std::collections::{BTreeMap, BTreeSet};

use pncad::document::{Doc, Frame, Node, ProfileProgram, RecipeNodeId};

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

/// A typed display-state refusal (closed enum, D4 ¶3). Every arm names
/// its subject; none is a message composed about another layer's
/// failure.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayFault {
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
    /// A free-move gesture operation arrived with no gesture in
    /// flight.
    NoFreeMove,
    /// A free-move gesture is already in flight.
    FreeMoveInFlight,
}

impl core::fmt::Display for DisplayFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
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
            Self::NonRigidFrame { determinant } => write!(
                f,
                "the free-move frame is not a finite rigid motion (determinant {determinant}); \
                 the probe admits rotations and translations only"
            ),
            Self::NoFreeMove => write!(f, "no free-move is in progress"),
            Self::FreeMoveInFlight => write!(f, "finish the free-move first"),
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
            Some(Node::Mate { a, b, .. }) => a.node == instance || b.node == instance,
            _ => false,
        })
        .collect()
}

/// Whether `node` is a live `InstantiatePart`.
pub fn is_instance(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    matches!(doc.node(node), Some(Node::InstantiatePart { .. }))
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
/// refuses [`DisplayFault::FusedGeometry`] — the op could not take
/// effect without moving material that is not the instance's, and an
/// accepted-but-inert op is the dishonesty G3 forbids.
///
/// # Errors
///
/// [`DisplayFault::NotAnInstance`], [`DisplayFault::FusedGeometry`].
pub fn drawn_targets(
    doc: &Doc<ProfileProgram>,
    instance: RecipeNodeId,
) -> Result<BTreeSet<RecipeNodeId>, DisplayFault> {
    if !is_instance(doc, instance) {
        return Err(DisplayFault::NotAnInstance { node: instance });
    }
    let mut targets = BTreeSet::new();
    for (root, instances) in instances_by_root(doc) {
        if !instances.contains(&instance) {
            continue;
        }
        if instances.len() > 1 {
            return Err(DisplayFault::FusedGeometry {
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
) -> Result<(), DisplayFault> {
    drawn_targets(doc, instance).map(|_| ())
}

/// The free-move admission test: [`display_check`] plus no mate names
/// the instance.
///
/// # Errors
///
/// [`DisplayFault::NotAnInstance`], [`DisplayFault::FusedGeometry`],
/// [`DisplayFault::MateConstrained`].
pub fn free_move_check(
    doc: &Doc<ProfileProgram>,
    instance: RecipeNodeId,
) -> Result<(), DisplayFault> {
    display_check(doc, instance)?;
    let mates = mates_naming(doc, instance);
    if mates.is_empty() {
        Ok(())
    } else {
        Err(DisplayFault::MateConstrained { instance, mates })
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

/// A free-move gesture in flight: layer-3 state only, exactly the G1
/// preview/commit shape one level up from the document — previews
/// replace one another, and the commit lands one value.
#[derive(Debug, Clone, PartialEq)]
struct FreeMoveGesture {
    /// The instance being probed.
    instance: RecipeNodeId,
    /// The last previewed frame — what a commit would land. `None`
    /// until the first preview, so an untouched gesture commits
    /// nothing.
    preview: Option<Frame>,
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

/// **The one home** for hide and free-move state (the seam-friction
/// inventory discipline: no per-widget shadows). Owned by the session;
/// every mutation is a typed operation routed through it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayState {
    hidden: BTreeSet<RecipeNodeId>,
    moves: BTreeMap<RecipeNodeId, Frame>,
    gesture: Option<FreeMoveGesture>,
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
        self.gesture.as_ref().map(|g| g.instance)
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
        if let Some(gesture) = &self.gesture
            && let Some(frame) = gesture.preview
        {
            moved.insert(gesture.instance, frame);
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
    /// [`DisplayFault::NotAnInstance`] — hiding is a per-instance
    /// operation; other node kinds draw through their own roots and
    /// have no instance identity to hide by — and
    /// [`DisplayFault::FusedGeometry`] for an instance the drawn
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
    /// # Errors
    ///
    /// [`DisplayFault::FreeMoveInFlight`], and [`free_move_check`]'s
    /// refusals.
    pub fn begin_free_move(
        &mut self,
        doc: &Doc<ProfileProgram>,
        instance: RecipeNodeId,
    ) -> Result<(), DisplayFault> {
        if self.gesture.is_some() {
            return Err(DisplayFault::FreeMoveInFlight);
        }
        free_move_check(doc, instance)?;
        self.gesture = Some(FreeMoveGesture {
            instance,
            preview: None,
        });
        Ok(())
    }

    /// Stream one preview frame into the in-flight gesture. Each
    /// preview REPLACES the last — the composed display value is
    /// `frame`, never an accumulation of deltas.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`], [`DisplayFault::NonRigidFrame`].
    pub fn preview_free_move(&mut self, frame: Frame) -> Result<(), DisplayFault> {
        let Some(gesture) = self.gesture.as_mut() else {
            return Err(DisplayFault::NoFreeMove);
        };
        if !is_rigid(&frame) {
            return Err(DisplayFault::NonRigidFrame {
                determinant: frame.determinant(),
            });
        }
        gesture.preview = Some(frame);
        self.revision += 1;
        Ok(())
    }

    /// Land the gesture: its last previewed frame becomes the
    /// instance's committed probe value. A gesture that never
    /// previewed commits nothing (the no-move rule the document
    /// gestures follow). A bit-exact identity commit REMOVES the
    /// entry: "probed to exactly where the document draws it" is the
    /// same picture as "not probed", and the distinctness treatment
    /// must not mark a part that is not displaced.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`].
    pub fn commit_free_move(&mut self) -> Result<(), DisplayFault> {
        let Some(gesture) = self.gesture.take() else {
            return Err(DisplayFault::NoFreeMove);
        };
        if let Some(frame) = gesture.preview {
            if frame.is_identity_bits() {
                self.moves.remove(&gesture.instance);
            } else {
                self.moves.insert(gesture.instance, frame);
            }
            self.revision += 1;
        }
        Ok(())
    }

    /// Abandon the gesture, restoring the committed picture.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NoFreeMove`].
    pub fn cancel_free_move(&mut self) -> Result<(), DisplayFault> {
        let had_preview = match self.gesture.take() {
            None => return Err(DisplayFault::NoFreeMove),
            Some(gesture) => gesture.preview.is_some(),
        };
        if had_preview {
            self.revision += 1;
        }
        Ok(())
    }

    /// Reconcile this state with a new document: DISCARD every probe
    /// whose instance is now mate-constrained, fused, or gone (the
    /// supersession rule, module docs), drop hidden entries whose
    /// instance the picture can no longer address, and kill an
    /// in-flight gesture whose instance became ineligible. Returns the
    /// instances whose COMMITTED probes were discarded, so a caller
    /// can report the supersession rather than infer it. A killed
    /// in-flight gesture is NOT in that list (it committed nothing);
    /// the caller observes it through [`DisplayState::probing`] going
    /// `None`, and the next gesture op refuses typed.
    pub fn prune(&mut self, doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
        let mut discarded = Vec::new();
        self.moves.retain(|&instance, _| {
            let keep = free_move_check(doc, instance).is_ok();
            if !keep {
                discarded.push(instance);
            }
            keep
        });
        let dead_hidden: Vec<RecipeNodeId> = self
            .hidden
            .iter()
            .copied()
            .filter(|&i| display_check(doc, i).is_err())
            .collect();
        for id in &dead_hidden {
            self.hidden.remove(id);
        }
        let gesture_dies = self
            .gesture
            .as_ref()
            .is_some_and(|g| free_move_check(doc, g.instance).is_err());
        if gesture_dies {
            self.gesture = None;
        }
        if !discarded.is_empty() || !dead_hidden.is_empty() || gesture_dies {
            self.revision += 1;
        }
        discarded
    }

    /// Forget everything — what opening a different document does.
    pub fn clear(&mut self) {
        if !self.hidden.is_empty() || !self.moves.is_empty() || self.gesture.is_some() {
            self.revision += 1;
        }
        self.hidden.clear();
        self.moves.clear();
        self.gesture = None;
    }
}
