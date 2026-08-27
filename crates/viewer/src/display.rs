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

/// The free-move admission test: a live instance that no mate names.
///
/// # Errors
///
/// [`DisplayFault::NotAnInstance`], [`DisplayFault::MateConstrained`].
pub fn free_move_check(
    doc: &Doc<ProfileProgram>,
    instance: RecipeNodeId,
) -> Result<(), DisplayFault> {
    if !is_instance(doc, instance) {
        return Err(DisplayFault::NotAnInstance { node: instance });
    }
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

/// What the scene and pick paths read: the hidden set and the
/// effective per-instance probe frames, snapshotted as values.
///
/// Owned rather than borrowed so a caller can hold one across the
/// mutation that would invalidate a borrow, and because both maps are
/// a handful of entries.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayView {
    /// Instances the picture drops.
    pub hidden: BTreeSet<RecipeNodeId>,
    /// Instance → the display frame composed over its drawn placement.
    /// An in-flight preview overrides that instance's committed value.
    pub moved: BTreeMap<RecipeNodeId, Frame>,
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

    /// The snapshot the scene and pick paths consume.
    pub fn view(&self) -> DisplayView {
        let mut moved = self.moves.clone();
        if let Some(gesture) = &self.gesture
            && let Some(frame) = gesture.preview
        {
            moved.insert(gesture.instance, frame);
        }
        DisplayView {
            hidden: self.hidden.clone(),
            moved,
        }
    }

    /// Hide or show one instance.
    ///
    /// # Errors
    ///
    /// [`DisplayFault::NotAnInstance`] — hiding is a per-instance
    /// operation; other node kinds draw through their own roots and
    /// have no instance identity to hide by.
    pub fn set_hidden(
        &mut self,
        doc: &Doc<ProfileProgram>,
        instance: RecipeNodeId,
        hidden: bool,
    ) -> Result<(), DisplayFault> {
        if !is_instance(doc, instance) {
            return Err(DisplayFault::NotAnInstance { node: instance });
        }
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
    /// whose instance is now mate-constrained or gone (the
    /// supersession rule, module docs), drop hidden entries for nodes
    /// no longer in the recipe, and kill an in-flight gesture whose
    /// instance became ineligible. Returns the instances whose probes
    /// were discarded, so a caller can report the supersession rather
    /// than infer it.
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
            .filter(|&i| !is_instance(doc, i))
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
