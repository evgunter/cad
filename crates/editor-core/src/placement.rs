//! Placement: the literal [`Frame`], and the [`Placement`] chain a
//! `Node::Transform`, a `Node::Gauge` and an instance's offset hold
//! (`crates/editor-core/ASSEMBLY.md` A11 (2)) — rigid steps of
//! expressions and literal frames.

use geom_core::predicate::Band;
use geom_core::{Affine3, Decide, Mat3, Real, Vec3};

use crate::eval::slots::SlotValues;
use crate::eval::{NodeErrorKind, NodeRefusal};
use crate::expr::{Expr, ParamEnv};
use crate::node::{RigidArg, SlotId};

/// **A placement axis with no definite direction** — the ONE thing
/// [`Frame::rotate_then_translate`] refuses, and the only thing
/// [`crate::EditError::PlacementAxis`] can be built from.
///
/// It is a type rather than a bare [`NodeErrorKind`] so that the
/// conversion into the authoring vocabulary is narrow: a blanket
/// `From<NodeErrorKind>` would let ANY node refusal reach a user
/// wearing the axis's words, which is the shape a refusal naming its
/// cause exists to avoid.
#[derive(Debug, Clone, PartialEq)]
pub struct AxisRefusal(NodeRefusal);

impl AxisRefusal {
    /// The direction door's refusal, as the evaluation layer typed it.
    #[must_use]
    pub fn kind(&self) -> &NodeErrorKind {
        self.0.kind()
    }

    /// The refusal, in the shape the document layer's error enums
    /// carry an evaluation refusal.
    #[must_use]
    pub fn carried(&self) -> NodeRefusal {
        self.0.clone()
    }
}

impl core::fmt::Display for AxisRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// The role word a PLACEMENT frame's rotation axis is normalized
/// under — the fourth of the direction roles, beside a transform's
/// rotation axis, a pattern's direction and a datum axis's. One
/// spelling, so the refusal a user reads names the vector they
/// authored rather than the frame it was going to build.
pub(crate) const PLACEMENT_AXIS_ROLE: &str = "placement rotation axis";

/// A placement frame: an affine map of world space, stored as the
/// linear part's COLUMNS (the images of the basis vectors, matching
/// [`Mat3::from_cols`]) plus the translation.
///
/// Stored as a general linear part rather than an axis–angle triple
/// because A6's improper frames (det = −1, the mirror case) must be
/// REPRESENTABLE in order to be refused: an axis–angle encoding has
/// determinant +1 identically, so the v1 refusal could never fire and
/// the R4 equivariance prerequisite would be invisible in the format.
///
/// Rigidity is not a field invariant — it is a decided predicate with
/// one home, [`topo::check_rigid`], which the kernel's placement door
/// ([`topo::transform_rigid`]) asks of every map it moves a body by and
/// [`Frame::admission_fault`] asks of every frame a document admits.
///
/// # Exactness
///
/// **The one home of this rule**; the methods below are cases of it and
/// do not restate it.
///
/// A frame's stored coordinates are CARRIED, never recomputed:
/// [`Frame::affine`] reads them back at any scalar as a structural map
/// through [`Real::from_f64`], so it is exact wherever that conversion
/// is and the identity at `f64`. Where a door does arithmetic the
/// claim is weaker, deliberately — D9-deterministic, not exact:
/// [`Mat3::determinant`]'s fixed evaluation order for the sign,
/// [`Affine3`]'s own product in that operator's fixed association for
/// [`Frame::compose`]'s general arm.
///
/// A BIT-exact identity is a fast path, and only a bit-exact one may
/// be, since any other value could round: [`Motion::compose`] — the one
/// composition rule, which [`Frame::compose`] and a [`Placement`]'s
/// chain both fold through — returns the other operand verbatim on one,
/// admitted by [`Frame::is_identity_bits`] over [`Frame::bit_eq`]. That
/// is what makes the split/inline round trip exact — the frames a split
/// hoists or leaves behind compose back with zero arithmetic, so D-4's
/// bit-level volume identity never meets a rounding step — and what
/// makes an identity step in a chain move no bit.
///
/// A placement and a modeled transform of the same part agree BIT FOR
/// BIT: [`Frame::rotate_then_translate`] is the `Transform` node's own
/// composition order (D9) built from the same expressions, down to
/// normalizing the axis on this side because that node does.
///
/// Every claim above is guarded, and each guard names the claim it
/// keeps rather than being listed here: one row per claim in this
/// module's `tests`, which is where to read what is actually pinned.
///
/// The exception is the bit agreement, whose guard needs a whole
/// document —
/// `r1_the_placement_frame_matches_the_transform_node_bit_for_bit` in
/// this crate's `asm2a_instantiate` suite. **A test function is not an
/// intra-doc link target, so that name is hand-written and no gate
/// reads it**: treat it as a hint and grep for the assertion.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    /// The linear part's columns: `columns[j]` is the image of basis
    /// vector `j`.
    pub columns: [[f64; 3]; 3],
    /// The translation — the image of the coordinate origin, as a
    /// displacement from it.
    pub translation: [f64; 3],
}

// Every method below returns a value and changes nothing, so
// discarding a result is always a bug: they all carry `#[must_use]`,
// the two private doors included, and a method added here does too.
// Nothing enforces that — `clippy::must_use_candidate` is off
// workspace-wide, and no test can read an attribute — so this line is
// the whole mechanism. `rotate_then_translate` is the one method
// without the attribute and needs none: its `Result` is `#[must_use]`
// by type, and repeating it there fires `clippy::double_must_use`
// unless given a message.
impl Frame {
    /// The identity frame.
    pub const IDENTITY: Self = Self {
        columns: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };

    /// The pure translation by `v`.
    #[must_use]
    pub fn translation(v: [f64; 3]) -> Self {
        Self {
            translation: v,
            ..Self::IDENTITY
        }
    }

    /// The rotation by `angle` radians (right-hand rule) about the axis
    /// through the origin with direction `axis`, then translation by
    /// `v` — the `Transform` node's own composition order (D9; see
    /// [`Frame`]'s exactness rule for what that order buys).
    ///
    /// **Normalizing the axis here is not redundant, and removing it
    /// would cost the bit agreement.** The transform node normalizes
    /// its axis before building the rotation (`eval::wire`'s `unit`)
    /// and `Mat3::rotation_about` normalizes again internally, so an
    /// axis carried through un-normalized would take one fewer rounding
    /// step through this door than through that one — for any axis, not
    /// just unit ones. Deciding the direction costs the agreement
    /// nothing, because [`geom_core::decide_unit_direction`] answers
    /// `v.normalize()`, the very expression a bare normalization used.
    ///
    /// **The axis is DECIDED here**, through the evaluation layer's
    /// own direction door ([`crate::eval::unit_direction`]) under
    /// [`PLACEMENT_AXIS_ROLE`] — the same door, and the same four
    /// answers below, that the transform node's axis takes. So the two
    /// constructions agree on their REFUSALS as well as on their bits,
    /// and a caller reads which vector of theirs was refused instead of
    /// being told the frame this door built is not finite.
    ///
    /// # Errors
    ///
    /// [`AxisRefusal`], carrying the direction door's own refusal
    /// unaltered ([`NodeErrorKind::DegenerateDirection`],
    /// [`NodeErrorKind::NonFiniteDirection`],
    /// [`NodeErrorKind::UnderflowedDirection`],
    /// [`NodeErrorKind::Escalated`]). [`crate::EditError::PlacementAxis`]
    /// is what carries it through the `SetOffset` door.
    pub fn rotate_then_translate(
        axis: [f64; 3],
        angle: f64,
        v: [f64; 3],
        band: Band,
    ) -> Result<Self, AxisRefusal> {
        let dir = crate::eval::unit_direction(
            Vec3::new(axis[0], axis[1], axis[2]),
            PLACEMENT_AXIS_ROLE,
            band,
        )
        .map_err(|e| AxisRefusal(NodeRefusal::from(e)))?;
        let m = Mat3::rotation_about(dir.get(), angle);
        Ok(Self {
            columns: [
                [m.c0.x, m.c0.y, m.c0.z],
                [m.c1.x, m.c1.y, m.c1.z],
                [m.c2.x, m.c2.y, m.c2.z],
            ],
            translation: v,
        })
    }

    /// The frame denoting an affine map — the inverse of
    /// [`Frame::affine`], for the mate solve's poses coming back from
    /// the coset algebra (ASM-R2a D-5).
    ///
    /// Every coordinate is CARRIED ([`Frame`]'s exactness rule), and
    /// nothing is snapped: a bit-exact identity map lands on
    /// [`Frame::IDENTITY`]'s own bits because those are the bits it
    /// arrived with — which is what an unmated instance's solved
    /// relative pose needs — and a map merely CLOSE to the identity
    /// keeps the coordinates it came in with.
    #[must_use]
    pub fn from_affine(a: geom_core::Affine3<f64>) -> Self {
        Self {
            columns: [
                [a.linear.c0.x, a.linear.c0.y, a.linear.c0.z],
                [a.linear.c1.x, a.linear.c1.y, a.linear.c1.z],
                [a.linear.c2.x, a.linear.c2.y, a.linear.c2.z],
            ],
            translation: [a.translation.x, a.translation.y, a.translation.z],
        }
    }

    /// Whether every stored coordinate is finite.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.columns
            .iter()
            .flatten()
            .chain(self.translation.iter())
            .all(|x| x.is_finite())
    }

    /// The linear part's determinant: a proper frame's is `+1`, an
    /// improper (mirroring) frame's `−1`.
    #[must_use]
    pub fn determinant(&self) -> f64 {
        self.linear_f64().determinant()
    }

    /// The linear part at the scalar it is STORED in — the one place
    /// the column arrays become a matrix.
    #[must_use]
    fn linear_f64(&self) -> Mat3<f64> {
        let col = |c: [f64; 3]| Vec3::new(c[0], c[1], c[2]);
        Mat3::from_cols(
            col(self.columns[0]),
            col(self.columns[1]),
            col(self.columns[2]),
        )
    }

    /// The affine map at the scalar it is STORED in — the one place
    /// the stored arrays become geometry.
    #[must_use]
    fn affine_f64(&self) -> Affine3<f64> {
        Affine3::from_parts(
            self.linear_f64(),
            Vec3::new(
                self.translation[0],
                self.translation[1],
                self.translation[2],
            ),
        )
    }

    /// The affine map this frame denotes, in the backend scalar — what
    /// the kernel's placement door consumes: the stored map through
    /// [`Affine3::map`] ([`Frame`]'s exactness rule).
    #[must_use]
    pub fn affine<T: Real>(&self) -> Affine3<T> {
        self.affine_f64().map(T::from_f64)
    }

    /// The motion this frame denotes in the backend scalar, with a
    /// bit-exact identity marked as such ([`Motion`]) — which only a
    /// stored frame can know, by its bits.
    #[must_use]
    pub(crate) fn motion<T: Real>(&self) -> Motion<T> {
        if self.is_identity_bits() {
            Motion::Identity
        } else {
            Motion::Map(self.affine())
        }
    }

    /// The frame a motion at `f64` denotes: the identity's own bits
    /// for the marked identity, the map's coordinates carried
    /// otherwise ([`Frame::from_affine`]).
    #[must_use]
    pub(crate) fn from_motion(motion: Motion<f64>) -> Self {
        match motion {
            Motion::Identity => Frame::IDENTITY,
            Motion::Map(map) => Frame::from_affine(map),
        }
    }

    /// The composition `self ∘ inner`: the frame that places by
    /// `inner` first, then by `self` — inline's rule (ASM-4 D-3: the
    /// instance's group frame composed onto the part's placements),
    /// by [`Motion::compose`] at `f64`.
    #[must_use]
    pub fn compose(&self, inner: &Frame) -> Frame {
        Frame::from_motion(self.motion::<f64>().compose(inner.motion()))
    }

    /// Whether this frame is the stored identity, BY BITS — D-3's
    /// admission test for the identity fast path ([`Frame`]'s exactness
    /// rule), which skips the kernel map.
    #[must_use]
    pub fn is_identity_bits(&self) -> bool {
        self.bit_eq(&Self::IDENTITY)
    }

    /// **The frame half of A11/A6's admission rule, stated once**: a
    /// frame the document admits carries only numbers, preserves
    /// orientation, and is a rigid motion.
    ///
    /// One predicate with one home, asked wherever a document admits a
    /// frame — a placement rule's listed frames
    /// ([`crate::node::PlacementRuleFault`]) and a placement's literal
    /// steps ([`Placement::frame_fault`]) — so none of them can come to
    /// hold frames to a different standard. Each caller keeps its own
    /// arms, says WHICH frame is at fault in its own vocabulary
    /// ([`FrameSite`]), and forwards this answer's sentence rather than
    /// restating it.
    ///
    /// Rigidity is [`topo::check_rigid`] at `tol`'s linear band, the
    /// predicate the evaluation's [`topo::TransformError::NotRigid`]
    /// applies, so a frame admitted here is one the evaluation moves a
    /// body by. A tolerance that forms no band decides nothing, and
    /// every evaluation at it refuses before it reads a frame, so the
    /// rigidity half is left to the next door that holds a band: the
    /// load door asks it again at the loading tolerance.
    ///
    /// It lives on [`Frame`] because the rule is about a frame and
    /// nothing else: it reads no document and no node.
    #[must_use]
    pub(crate) fn admission_fault(&self, tol: geom_core::Tol) -> Option<FrameFault> {
        if !self.is_finite() {
            return Some(FrameFault::NonFinite);
        }
        let determinant = self.determinant();
        if determinant <= 0.0 {
            return Some(FrameFault::Improper { determinant });
        }
        let band = Band::linear(tol).ok()?;
        match topo::check_rigid(&self.affine_f64(), band) {
            Ok(()) => None,
            Err(topo::TransformError::NotRigid { check }) => Some(FrameFault::NotRigid { check }),
            // `check_rigid` raises `NotRigid` and `NonFiniteMap`, and
            // the frame's coordinates were decided finite above.
            Err(_) => Some(FrameFault::NonFinite),
        }
    }

    /// Bit-semantic frame equality (D7's comparator family): every
    /// coordinate compares by BITS, so `±0.0` do not conflate.
    #[must_use]
    pub fn bit_eq(&self, other: &Self) -> bool {
        self.columns
            .iter()
            .flatten()
            .zip(other.columns.iter().flatten())
            .all(|(a, b)| a.to_bits() == b.to_bits())
            && self
                .translation
                .iter()
                .zip(other.translation.iter())
                .all(|(a, b)| a.to_bits() == b.to_bits())
    }
}

/// What makes a [`Frame`] inadmissible as a placement
/// ([`Frame::admission_fault`]) — one vocabulary, and one SENTENCE, for
/// every door that admits a frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameFault {
    /// A coordinate that is not a number: no predicate downstream can
    /// decide anything about where this frame puts the material.
    NonFinite,
    /// The frame is IMPROPER — determinant ≤ 0, i.e. a mirror (A6).
    /// Admitting one is gated on the equivariance audit R4 owns.
    Improper {
        /// The linear part's determinant.
        determinant: f64,
    },
    /// The frame is proper but not definitely a rigid motion at
    /// tolerance: it may scale or shear an axis
    /// ([`topo::check_rigid`]).
    NotRigid {
        /// The rigidity check that refused, which
        /// [`topo::not_rigid_reading`] puts in words; routing, never
        /// rendered by name.
        check: &'static str,
    },
}

// The ONE prose for the frame rule: a predicate clause, so each door
// forwards it into its own subject ([`FrameSite::subject`]) rather than
// inventing a second wording for the fact they share.
impl core::fmt::Display for FrameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite => f.write_str("carries a non-finite coordinate"),
            Self::Improper { determinant } => {
                write!(f, "is improper (mirroring): determinant {determinant}")
            }
            Self::NotRigid { check } => write!(
                f,
                "is not definitely rigid at tolerance: {}",
                topo::not_rigid_reading(check)
            ),
        }
    }
}

impl FrameFault {
    /// **What to do about the frame**, from any door that writes one:
    /// each refuses the edit that carried the frame, so making that edit
    /// again with the frame repaired gets through.
    #[must_use]
    pub fn recourse(&self) -> &'static str {
        match self {
            Self::NonFinite => "give every coordinate of that frame a finite value",
            Self::Improper { .. } => {
                "use a frame that does not mirror, a rotation and a translation; mirrored \
                 placements are admitted only behind the equivariance audit"
            }
            Self::NotRigid { .. } => "use only a rotation and a translation in that frame",
        }
    }
}

/// **Which frame of a node a placement refusal is about** — the subject
/// a door names beside the [`FrameFault`] sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSite {
    /// Listed placement `index` of an explicit placement rule.
    Listed {
        /// Its index in the placement list.
        index: usize,
    },
    /// Step `index` of the placement chain a node holds: a
    /// transform's, a gauge's, or an instance's offset.
    Step {
        /// Its index in the chain.
        index: usize,
    },
}

impl FrameSite {
    /// The frame this site names on `node`, as a sentence's subject. A
    /// step reads as a user counts, from one, as [`crate::SlotId::label`]
    /// numbers a chain's steps; a listed placement reads by its index,
    /// the one an instance's name carries.
    #[must_use]
    pub fn subject(self, node: crate::node::RecipeNodeId) -> String {
        match self {
            Self::Listed { index } => format!("placement {index} of node {}", node),
            Self::Step { index } => format!("step {} of node {}'s placement", index + 1, node),
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// **A motion with its bit-exact identity marked** — the one
/// composition rule ([`Frame`]'s exactness rule), generic over the
/// scalar so a [`Placement`]'s chain folds through the same rule
/// [`Frame::compose`] does at `f64`.
///
/// The identity is marked rather than detected: only a stored frame
/// knows it is the identity, by its bits ([`Frame::motion`]); a map
/// computed from expressions is a [`Motion::Map`] whatever its value.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Motion<T: Real> {
    /// The bit-exact identity: composes with anything by returning it.
    Identity,
    /// Any other motion.
    Map(Affine3<T>),
}

impl<T: Real> Motion<T> {
    /// `self ∘ inner`, `inner` acting first. A bit-exact identity on
    /// either side returns the other operand verbatim, with zero
    /// arithmetic; otherwise [`Affine3`]'s own product in its fixed
    /// association.
    #[must_use]
    pub(crate) fn compose(self, inner: Self) -> Self {
        match (self, inner) {
            (Self::Identity, other) | (other, Self::Identity) => other,
            (Self::Map(outer), Self::Map(inner)) => Self::Map(outer * inner),
        }
    }

    /// The map, the identity included.
    #[must_use]
    pub(crate) fn affine(self) -> Affine3<T> {
        match self {
            Self::Identity => Affine3::identity(),
            Self::Map(map) => map,
        }
    }

    /// The map, `None` for the bit-exact identity — what a door that
    /// skips the identity (an instance placement's fast path) reads.
    #[must_use]
    pub(crate) fn non_identity(self) -> Option<Affine3<T>> {
        match self {
            Self::Identity => None,
            Self::Map(map) => Some(map),
        }
    }
}

/// **A placement: an ordered chain of steps** — what a
/// [`crate::Node::Transform`] and a [`crate::Node::Gauge`] hold, and an
/// instance's offset is (ASSEMBLY-DESIGN A11 (2)). An explicit rule's
/// listed placements hold literal [`Frame`]s.
///
/// The chain composes as a product: `[s0, s1, …, sn]` denotes
/// `s0 ∘ s1 ∘ … ∘ sn`, the reading A11 (5) writes "gauge frame ∘ offset
/// ∘ solved pose" in and [`Frame::compose`] computes. Each step is
/// expressed in the frame the steps before it build, and the last step
/// is the first to act on the placed body: a `point_at` frame composed
/// with a rigid step spins the body inside the aimed frame.
/// [`Placement::compose`] is the door that builds one chain from two.
///
/// The empty chain is the identity ([`Placement::IDENTITY`]), the unit
/// of [`Placement::compose`]; both doors admit it.
///
/// A rigid step is parametric: its components are slot expressions,
/// addressed per step ([`crate::SlotId::rigid`]). A literal step
/// carries no slot.
///
/// # Exactness
///
/// [`Placement::eval`] builds a rigid step by the construction the
/// transform node has always used, so a one-step rigid placement moves
/// a body by exactly the bits it did before the chain existed; a
/// literal step is read through [`Frame::motion`], so a one-step
/// literal placement IS its frame, bit for bit ([`Placement::literal`]).
/// The chain folds through [`Motion::compose`], so a bit-exact identity
/// literal anywhere in it moves no bit; any other chain of two or more
/// steps is the [`Affine3`] product, which rounds as that product does.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    /// The steps, composed as a product (`[a, b]` is `a ∘ b`).
    pub steps: Vec<Step>,
}

/// One step of a [`Placement`].
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Step {
    /// Rotate by `angle` about the axis through the ORIGIN with
    /// direction `axis`, then translate by `translation` — proper by
    /// construction. The axis is decided at evaluation by the
    /// evaluation layer's direction door, which refuses one of no
    /// definite direction.
    Rigid {
        /// Translation components, Length.
        translation: [Expr; 3],
        /// Rotation-axis components, Scalar.
        axis: [Expr; 3],
        /// Rotation angle, Angle.
        angle: Expr,
    },
    /// A literal frame, held to [`Frame::admission_fault`] at every door
    /// that writes one and at load.
    Literal(Frame),
}

impl From<Step> for Placement {
    /// The one-step placement.
    fn from(step: Step) -> Self {
        Self { steps: vec![step] }
    }
}

impl Placement {
    /// The empty chain: the identity, and the unit of
    /// [`Placement::compose`].
    pub const IDENTITY: Self = Self { steps: Vec::new() };

    /// **The one bit-exact `Frame` → `Placement` door**: the one-step
    /// literal placement, which evaluates to `frame` bit for bit at
    /// `f64`. The frame is carried, not checked: admission is the
    /// doors' ([`Placement::frame_fault`]).
    #[must_use]
    pub fn literal(frame: &Frame) -> Self {
        Step::Literal(*frame).into()
    }

    /// The composition `self ∘ inner`, in [`Frame::compose`]'s order:
    /// `inner`'s steps act on the body first, expressed in the frame
    /// `self` builds. The chain is `self`'s steps followed by
    /// `inner`'s.
    #[must_use]
    pub fn compose(&self, inner: &Placement) -> Placement {
        Placement {
            steps: self.steps.iter().chain(&inner.steps).cloned().collect(),
        }
    }

    /// The first literal step [`Frame::admission_fault`] refuses at
    /// `tol`, with its index in the chain.
    #[must_use]
    pub fn frame_fault(&self, tol: geom_core::Tol) -> Option<(usize, FrameFault)> {
        self.steps
            .iter()
            .enumerate()
            .find_map(|(k, step)| match step {
                Step::Literal(frame) => Some((k, frame.admission_fault(tol)?)),
                Step::Rigid { .. } => None,
            })
    }

    /// Bit-semantic equality (D7): a rigid step's expressions through
    /// [`Expr::bit_eq`], a literal step's coordinates through
    /// [`Frame::bit_eq`], so `0.0` and `-0.0` are different
    /// placements.
    #[must_use]
    pub fn bit_eq(&self, other: &Self) -> bool {
        self.steps.len() == other.steps.len()
            && self
                .steps
                .iter()
                .zip(&other.steps)
                .all(|(a, b)| match (a, b) {
                    (
                        Step::Rigid {
                            translation: ta,
                            axis: aa,
                            angle: ga,
                        },
                        Step::Rigid {
                            translation: tb,
                            axis: ab,
                            angle: gb,
                        },
                    ) => {
                        ta.iter()
                            .chain(aa)
                            .zip(tb.iter().chain(ab))
                            .all(|(x, y)| x.bit_eq(y))
                            && ga.bit_eq(gb)
                    }
                    (Step::Literal(fa), Step::Literal(fb)) => fa.bit_eq(fb),
                    (Step::Rigid { .. }, Step::Literal(_))
                    | (Step::Literal(_), Step::Rigid { .. }) => false,
                })
    }

    /// **The rigid motion this placement denotes, at `env`** — every
    /// rigid step's expressions evaluated through the evaluation's one
    /// slot door (`eval::slots::eval_rows`), then the construction the
    /// transform node's evaluation and the mate solve read.
    ///
    /// # Errors
    ///
    /// [`NodeErrorKind::Expr`] naming the first slot that does not
    /// evaluate, and the direction door's own refusal for an axis of
    /// no definite direction.
    pub fn eval<T: Decide>(
        &self,
        env: &ParamEnv<T>,
        band: Band,
    ) -> Result<Affine3<T>, NodeErrorKind> {
        self.motion_at(env, band).map(Motion::affine)
    }

    /// [`Placement::eval`] with the bit-exact identity kept marked
    /// ([`Motion`]): the empty chain and a chain of identity literals
    /// are [`Motion::Identity`], so a placement that moves nothing
    /// composes with no arithmetic.
    ///
    /// # Errors
    ///
    /// [`Placement::eval`]'s.
    pub(crate) fn motion_at<T: Decide>(
        &self,
        env: &ParamEnv<T>,
        band: Band,
    ) -> Result<Motion<T>, NodeErrorKind> {
        let vals = crate::eval::slots::eval_rows(self.rows(), env)
            .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
        self.chain_motion(&vals, band)
    }

    /// **The one construction of a placement's motion**, from its
    /// slots already evaluated: a rigid step by the transform
    /// construction over its axis decided under the transform's role
    /// word, a literal step by [`Frame::motion`], the chain folded
    /// through [`Motion::compose`].
    ///
    /// # Errors
    ///
    /// [`NodeErrorKind::MissingSlot`] when a rigid step's slot is not
    /// among `vals`, and the direction door's own refusal.
    pub(crate) fn motion<T: Decide>(
        &self,
        vals: &SlotValues<T>,
        band: Band,
    ) -> Result<Affine3<T>, NodeErrorKind> {
        self.chain_motion(vals, band).map(Motion::affine)
    }

    /// [`Placement::motion`] with the identity kept marked.
    fn chain_motion<T: Decide>(
        &self,
        vals: &SlotValues<T>,
        band: Band,
    ) -> Result<Motion<T>, NodeErrorKind> {
        let mut composed = Motion::Identity;
        for (k, step) in self.steps.iter().enumerate() {
            let map = match step {
                Step::Rigid { .. } => {
                    let translation = crate::eval::need_vec3(vals, |ax| {
                        SlotId::rigid(k, RigidArg::Translation(ax))
                    })?;
                    let axis = crate::eval::unit_direction(
                        crate::eval::need_vec3(vals, |ax| {
                            SlotId::rigid(k, RigidArg::RotationAxis(ax))
                        })?,
                        crate::eval::TRANSFORM_AXIS_ROLE,
                        band,
                    )?;
                    let angle =
                        crate::eval::need_scalar(vals, SlotId::rigid(k, RigidArg::RotationAngle))?;
                    Motion::Map(crate::eval::transform_map(translation, axis, angle))
                }
                Step::Literal(frame) => frame.motion(),
            };
            composed = composed.compose(map);
        }
        Ok(composed)
    }
}

#[cfg(test)]
mod tests {
    //! [`Frame`]'s exactness rule, at the stored-arrays-to-geometry
    //! doors — which the eval-level tests exercise only through whole
    //! documents — and [`Placement`]'s, at its one-step chains.
    #![allow(clippy::expect_used)]

    use super::*;

    /// A frame with no symmetry between its components, so a
    /// transposed or permuted product cannot pass by coincidence.
    ///
    /// The `-0.0` in `columns[0]` and in `translation` is LOAD-BEARING
    /// and must survive any edit to this fixture: a signed zero is the
    /// only value whose bits `x + 0.0` changes, so it is what
    /// separates a door that COPIES the stored coordinate from one
    /// that rebuilds it arithmetically, and what makes
    /// [`Frame::compose`]'s identity fast path observable at all
    /// (without it the general arm returns the same bits).
    fn sample() -> Frame {
        Frame {
            columns: [
                [0.5, -0.0, 3.0],
                [1.0e-9, 2.0, -0.125],
                [-7.0, 0.75, 1.0 / 3.0],
            ],
            translation: [1.0e12, -0.0, 0.1],
        }
    }

    fn other() -> Frame {
        Frame {
            columns: [
                [2.0, 0.3, -1.5],
                [-0.125, 1.0 / 7.0, 4.0],
                [9.0, -2.5, 0.25],
            ],
            translation: [-3.0, 0.5, 1.0e-13],
        }
    }

    /// The bit patterns a COPY carries and a computation does not: a
    /// NaN with a payload, both zeros, a subnormal and both
    /// infinities. Only the reading doors are asked about it —
    /// arithmetic over these values is not a claim this module makes.
    fn bit_zoo() -> Frame {
        let nan = f64::from_bits(0x7FF8_0000_DEAD_BEEF);
        Frame {
            columns: [
                [nan, 0.0, -0.0],
                [f64::INFINITY, f64::NEG_INFINITY, 5.0e-324],
                [-5.0e-324, f64::MIN_POSITIVE, -1.0],
            ],
            translation: [-0.0, f64::from_bits(0x000F_FFFF_FFFF_FFFF), f64::MAX],
        }
    }

    /// `outer ∘ inner` written out as the thirty-six scalar operations
    /// the kernel performs, in the association its operators fix:
    /// every product column is `(a.c0·x + a.c1·y) + a.c2·z`
    /// ([`Mat3`]'s matrix–vector order applied to the inner column),
    /// and the outer frame's own translation is added last. It shares
    /// no operator with its subject, so a reassociation ANYWHERE under
    /// [`Frame::compose`] — in `Affine3`'s product or in `Mat3`'s —
    /// moves the two apart.
    fn composed_by_hand(outer: &Frame, inner: &Frame) -> Frame {
        let mut columns = [[0.0f64; 3]; 3];
        for (j, out) in columns.iter_mut().enumerate() {
            for (i, x) in out.iter_mut().enumerate() {
                *x = outer.columns[0][i] * inner.columns[j][0]
                    + outer.columns[1][i] * inner.columns[j][1]
                    + outer.columns[2][i] * inner.columns[j][2];
            }
        }
        let mut translation = [0.0f64; 3];
        for (i, x) in translation.iter_mut().enumerate() {
            *x = outer.columns[0][i] * inner.translation[0]
                + outer.columns[1][i] * inner.translation[1]
                + outer.columns[2][i] * inner.translation[2]
                + outer.translation[i];
        }
        Frame {
            columns,
            translation,
        }
    }

    /// Keeps the CARRIED-not-recomputed claim: a read-back at `f64`
    /// moves no bits.
    #[test]
    fn affine_at_f64_carries_the_stored_bits() {
        for f in [sample(), bit_zoo()] {
            let a = f.affine::<f64>();
            for (j, c) in [a.linear.c0, a.linear.c1, a.linear.c2]
                .into_iter()
                .enumerate()
            {
                for (i, x) in [c.x, c.y, c.z].into_iter().enumerate() {
                    assert_eq!(
                        x.to_bits(),
                        f.columns[j][i].to_bits(),
                        "column {j} entry {i}"
                    );
                }
            }
            for (i, x) in [a.translation.x, a.translation.y, a.translation.z]
                .into_iter()
                .enumerate()
            {
                assert_eq!(x.to_bits(), f.translation[i].to_bits(), "translation {i}");
            }
        }
    }

    /// Keeps the D9-deterministic claim for [`Frame::compose`]: the
    /// general arm is [`Affine3`]'s product in that operator's fixed
    /// association, and a reassociation anywhere under it reddens this.
    #[test]
    fn compose_is_the_affine_product_to_the_last_multiply_add() {
        let (a, b) = (sample(), other());
        assert!(a.compose(&b).bit_eq(&composed_by_hand(&a, &b)));
        assert!(b.compose(&a).bit_eq(&composed_by_hand(&b, &a)));
    }

    /// Keeps the D9-deterministic claim for [`Frame::determinant`]: it
    /// is [`Mat3::determinant`]'s own association, `c0 · (c1 × c2)`
    /// with the dot summed left to right, and NOT merely the right
    /// value.
    ///
    /// The fixture makes the three summands `1.0`, `1e16` and `-1e16`,
    /// where the two groupings of one addition chain disagree by a
    /// whole unit: `(1 + 1e16) - 1e16` is `0.0`, `1 + (1e16 - 1e16)` is
    /// `1.0`. So the second assertion is what gives the row teeth — a
    /// reassociation inside [`Vec3::dot`] or [`Vec3::cross`], or a
    /// [`Frame::determinant`] that stops delegating, moves the answer
    /// onto the value this row refuses.
    #[test]
    fn determinant_is_mat3s_association_and_not_just_its_value() {
        let f = Frame {
            columns: [[1.0, -1.0e16, -1.0e16], [1.0, 1.0, 0.0], [0.0, 1.0, 1.0]],
            translation: [0.0, 0.0, 0.0],
        };
        // `c0.dot(c1.cross(c2))`, written out in that fixed order.
        let [c0, c1, c2] = f.columns;
        let cross = [
            c1[1] * c2[2] - c1[2] * c2[1],
            c1[2] * c2[0] - c1[0] * c2[2],
            c1[0] * c2[1] - c1[1] * c2[0],
        ];
        let in_order = (c0[0] * cross[0] + c0[1] * cross[1]) + c0[2] * cross[2];
        let regrouped = c0[0] * cross[0] + (c0[1] * cross[1] + c0[2] * cross[2]);
        assert_ne!(
            in_order.to_bits(),
            regrouped.to_bits(),
            "fixture is vacuous: the two groupings agree"
        );
        assert_eq!(f.determinant().to_bits(), in_order.to_bits());
    }

    /// Keeps the fast-path claim: a bit-exact identity on either side
    /// returns the other operand with zero arithmetic.
    #[test]
    fn compose_with_an_identity_returns_the_other_operand_verbatim() {
        let f = sample();
        assert!(Frame::IDENTITY.compose(&f).bit_eq(&f));
        assert!(f.compose(&Frame::IDENTITY).bit_eq(&f));
    }

    /// Keeps [`Frame::from_affine`]'s CARRIED-not-recomputed claim in
    /// the direction [`Frame::affine`]'s row does not cover: the
    /// coordinates of the affine arrive with their bits intact, and
    /// NOTHING is snapped on the way in.
    ///
    /// The teeth are the `is_identity_bits` assertions, and the
    /// fixtures under them must perturb the identity in BOTH PARTS —
    /// a translation-only set leaves a linear-part snap green, and a
    /// linear-part snap is the one that silently changes an answer: a
    /// solved pose composes through [`Frame::motion`], whose identity
    /// arm DISCARDS the pose, so an instance that rotated by a hair
    /// would read as "did not move".
    ///
    /// Each of the four is one representable step from the identity —
    /// a signed zero, a subnormal, an off-diagonal subnormal, and the
    /// next `f64` below `1.0` — so any tolerance a door could adopt
    /// swallows all four.
    #[test]
    fn from_affine_carries_the_affines_bits_and_snaps_nothing() {
        let mut signed_zero = Frame::IDENTITY;
        signed_zero.translation[0] = -0.0;
        let mut subnormal = Frame::IDENTITY;
        subnormal.translation[2] = 5.0e-324;
        // Zero translation, so only the LINEAR part is off the
        // identity: a subnormal shear, and one ulp of scale.
        let mut sheared = Frame::IDENTITY;
        sheared.columns[0][1] = 5.0e-324;
        let mut scaled = Frame::IDENTITY;
        scaled.columns[2][2] = 1.0 - f64::EPSILON / 2.0;

        let near_identity = [
            ("-0.0 translation", signed_zero),
            ("subnormal translation", subnormal),
            ("subnormal shear", sheared),
            ("one ulp of scale", scaled),
        ];

        for (name, f) in near_identity.iter().copied().chain([
            ("identity", Frame::IDENTITY),
            ("sample", sample()),
            ("other", other()),
            ("bit zoo", bit_zoo()),
        ]) {
            assert!(
                Frame::from_affine(f.affine_f64()).bit_eq(&f),
                "from_affine moved a bit at {name}"
            );
        }

        assert!(Frame::from_affine(Frame::IDENTITY.affine_f64()).is_identity_bits());
        for (name, f) in near_identity {
            assert!(
                !Frame::from_affine(f.affine_f64()).is_identity_bits(),
                "from_affine snapped {name} onto the identity"
            );
            // The fixture is one step from the identity, not equal to
            // it: a vacuous row would pass the assertion above.
            assert!(
                !f.bit_eq(&Frame::IDENTITY),
                "fixture {name} IS the identity"
            );
        }
    }

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).expect("the witness tolerance forms a band")
    }

    fn motion_bits(a: &Affine3<f64>) -> Vec<u64> {
        [a.linear.c0, a.linear.c1, a.linear.c2, a.translation]
            .iter()
            .flat_map(|c| [c.x.to_bits(), c.y.to_bits(), c.z.to_bits()])
            .collect()
    }

    /// Keeps [`Placement::literal`]'s claim: a one-step literal
    /// evaluates to its frame bit for bit — the `-0.0`s of [`sample`]
    /// included, which any arithmetic on the way would turn to `+0.0`.
    #[test]
    fn a_one_step_literal_is_its_frame_bit_for_bit() {
        for f in [sample(), other()] {
            let motion = Placement::literal(&f)
                .eval::<f64>(&ParamEnv::default(), band())
                .expect("a literal evaluates");
            assert!(Frame::from_affine(motion).bit_eq(&f));
        }
    }

    /// Keeps [`Placement::eval`]'s claim for a one-step rigid chain: it
    /// is the transform construction itself, bit for bit. A zero angle
    /// puts `-0.0` in the rotation, and the translation carries one,
    /// so an identity multiplied in anywhere moves a bit.
    #[test]
    fn a_one_step_rigid_is_the_transform_construction_bit_for_bit() {
        use crate::test_support::{ang, len, scl};
        for (t, axis, angle) in [
            ([-0.0, 2.0, 0.1], [0.0, 0.0, 1.0], 0.0),
            ([1.0, -0.0, 3.0], [1.0, 2.0, -3.0], 0.7),
        ] {
            let placement = Placement::from(Step::Rigid {
                translation: t.map(len),
                axis: axis.map(scl),
                angle: ang(angle),
            });
            let got = placement
                .eval::<f64>(&ParamEnv::default(), band())
                .expect("a rigid step evaluates");
            let want = crate::eval::transform_map(
                Vec3::new(t[0], t[1], t[2]),
                crate::eval::unit_direction(
                    Vec3::new(axis[0], axis[1], axis[2]),
                    crate::eval::TRANSFORM_AXIS_ROLE,
                    band(),
                )
                .expect("a definite axis"),
                angle,
            );
            assert_eq!(motion_bits(&got), motion_bits(&want), "axis {axis:?}");
        }
    }
}
