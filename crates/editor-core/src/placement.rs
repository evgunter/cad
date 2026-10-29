//! Placement: the literal [`Frame`], the evaluated value of a rigid
//! motion, which an explicit pattern rule lists until patterns are
//! index variables.

use geom_core::predicate::Band;
use geom_core::{Affine3, Mat3, Real, RigidMotion, RigidMotionError, Vec3};

use crate::eval::{NodeErrorKind, NodeRefusal};

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
/// under, beside a pattern's direction and an axis pose's. One
/// spelling, so the refusal a user reads names the vector they
/// authored rather than the frame it was going to build.
pub(crate) const PLACEMENT_AXIS_ROLE: &str = "placement rotation axis";

/// A placement frame: an affine map of world space, stored as the
/// linear part's COLUMNS (the images of the basis vectors, matching
/// [`Mat3::from_cols`]) plus the translation.
///
/// Stored as a general linear part, so a frame that is not a rigid
/// motion (a scale, a shear, a mirror) is representable and refused:
/// every pose is proper, and a mirror is a construction
/// ([`crate::Node::Mirror`]).
///
/// Rigidity is not a field invariant — it is a decided predicate with
/// one home, [`RigidMotion::check_rigid`], which a listed frame asks
/// when it evaluates and [`Frame::admission_fault`] asks of every frame
/// a document admits.
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
/// be, since any other value could round: [`Frame::compose`] returns
/// the other operand verbatim on one, admitted by
/// [`Frame::is_identity_bits`] over [`Frame::bit_eq`]. That is what
/// makes the split/inline round trip exact — the offsets a split moves
/// compose back with zero arithmetic, so D-4's bit-level volume
/// identity never meets a rounding step.
///
/// Every claim above is guarded, and each guard names the claim it
/// keeps rather than being listed here: one row per claim in this
/// module's `tests`, which is where to read what is actually pinned.
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
    /// `v` (D9's composition order).
    ///
    /// **The axis is DECIDED here**, through the evaluation layer's
    /// own direction door ([`crate::eval::unit_direction`]) under
    /// [`PLACEMENT_AXIS_ROLE`], and the frame rotates about that
    /// witness as given ([`Mat3::rotation_about_unit`]). So a caller
    /// reads which vector of theirs was refused instead of being told
    /// the frame this door built is not finite.
    ///
    /// # Errors
    ///
    /// [`AxisRefusal`], carrying the direction door's own refusal
    /// unaltered ([`NodeErrorKind::DegenerateDirection`],
    /// [`NodeErrorKind::NonFiniteDirection`],
    /// [`NodeErrorKind::UnderflowedDirection`],
    /// [`NodeErrorKind::Escalated`]). [`crate::EditError::PlacementAxis`]
    /// is what carries it through the `AddConstraint` door.
    pub fn rotate_then_translate(
        axis: [f64; 3],
        angle: f64,
        v: [f64; 3],
        band: Band,
    ) -> Result<Self, AxisRefusal> {
        let dir = crate::eval::unit_direction(Vec3::from_array(axis), PLACEMENT_AXIS_ROLE, band)
            .map_err(|e| AxisRefusal(NodeRefusal::from(e)))?;
        Ok(Self {
            columns: Mat3::rotation_about_unit(dir, angle).to_cols_array(),
            translation: v,
        })
    }

    /// **[`Frame::rotate_then_translate`] by `turns` of a turn**: the
    /// rotation's sine and cosine read from the fraction
    /// ([`geom_core::Angle`]), so a whole number of quarter turns about a
    /// coordinate axis has columns of `0` and `±1` exactly, where the
    /// rounded radian's carry `6.1e-17`. The axis is decided as
    /// [`Frame::rotate_then_translate`] decides it.
    ///
    /// # Errors
    ///
    /// [`AxisRefusal`], as [`Frame::rotate_then_translate`]'s.
    #[allow(clippy::cast_precision_loss)]
    pub fn rotate_by_turns_then_translate(
        axis: [f64; 3],
        turns: geom_core::Turns,
        v: [f64; 3],
        band: Band,
    ) -> Result<Self, AxisRefusal> {
        let dir = crate::eval::unit_direction(Vec3::from_array(axis), PLACEMENT_AXIS_ROLE, band)
            .map_err(|e| AxisRefusal(NodeRefusal::from(e)))?;
        // The value the fraction stands for; the rotation reads the
        // fraction itself.
        let radians = core::f64::consts::TAU * (turns.num() as f64) / (turns.den() as f64);
        Ok(Self {
            columns: Mat3::rotation_about_unit_by(dir, geom_core::Angle::of_turns(radians, turns))
                .to_cols_array(),
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
        let Affine3 {
            linear,
            translation,
        } = a;
        Self {
            columns: linear.to_cols_array(),
            translation: translation.to_array(),
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

    /// The linear part's determinant: a rigid frame's is `+1`.
    #[must_use]
    pub fn determinant(&self) -> f64 {
        self.linear_f64().determinant()
    }

    /// The linear part at the scalar it is STORED in — the one place
    /// the column arrays become a matrix.
    #[must_use]
    fn linear_f64(&self) -> Mat3<f64> {
        Mat3::from_cols_array(self.columns)
    }

    /// The affine map at the scalar it is STORED in — the one place
    /// the stored arrays become geometry.
    #[must_use]
    fn affine_f64(&self) -> Affine3<f64> {
        Affine3::from_parts(self.linear_f64(), Vec3::from_array(self.translation))
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
    /// stored frame can know, by its bits. A general map
    /// ([`Frame::compose`] composes any two).
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
    /// `inner` first, then by `self`, by [`Motion::compose`] at `f64`.
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
    /// frame the document admits carries only numbers and is a rigid
    /// motion, so it preserves orientation.
    ///
    /// One predicate with one home, asked wherever a frame is admitted
    /// — the viewer's drawn frame — so no door can come to hold frames to
    /// a different standard. Its caller says WHICH frame
    /// is at fault in its own vocabulary, and forwards
    /// this answer's sentence rather than restating it.
    ///
    /// Rigidity is [`RigidMotion::check_rigid`] at `tol`'s linear band,
    /// the door a listed frame's evaluation asks, so a frame admitted
    /// here is one the evaluation moves a body by. A determinant that is
    /// not positive is refused first, at any tolerance: a reflection is
    /// no pose (every pose is proper; a mirror is
    /// [`crate::Node::Mirror`]). A tolerance that forms no band decides
    /// nothing else, and every evaluation at it refuses before it reads
    /// a frame, so the rest of rigidity is left to the next door that
    /// holds a band: the load door asks it again at the loading
    /// tolerance.
    ///
    /// It lives on [`Frame`] because the rule is about a frame and
    /// nothing else: it reads no document and no node.
    #[must_use]
    pub fn admission_fault(&self, tol: geom_core::Tol) -> Option<FrameFault> {
        if !self.is_finite() {
            return Some(FrameFault::NonFinite);
        }
        // A reflection is refused whatever the tolerance: no band reads
        // the sign of a determinant that is not positive.
        if self.determinant() <= 0.0 {
            return Some(FrameFault::NotRigid {
                check: "transform_rigid_det_plus_one",
            });
        }
        let band = Band::linear(tol).ok()?;
        match RigidMotion::check_rigid(&self.affine_f64(), band) {
            Ok(_) => None,
            Err(
                RigidMotionError::NotRigid { check } | RigidMotionError::Undecided { check, .. },
            ) => Some(FrameFault::NotRigid { check }),
            // The door's other refusals are about finiteness, and the
            // frame's coordinates were decided finite above.
            Err(RigidMotionError::NonFinite(_)) => Some(FrameFault::NonFinite),
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
    /// The frame is not definitely a rigid motion at tolerance: it may
    /// scale, shear or mirror an axis ([`RigidMotion::check_rigid`]).
    NotRigid {
        /// The rigidity check that refused, which
        /// [`geom_core::not_rigid_reading`] puts in words; routing,
        /// never rendered by name.
        check: &'static str,
    },
}

// The ONE prose for the frame rule: a predicate clause, so each door
// forwards it into its own subject rather than
// inventing a second wording for the fact they share.
impl core::fmt::Display for FrameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite => f.write_str("carries a non-finite coordinate"),
            Self::NotRigid { check } => write!(
                f,
                "is not definitely rigid at tolerance: {}",
                geom_core::not_rigid_reading(check)
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
            Self::NotRigid { .. } => "use only a rotation and a translation in that frame",
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// **A motion with its bit-exact identity marked** — [`Frame`]'s
/// composition rule, which [`Frame::compose`] folds through.
///
/// The identity is marked rather than detected: only a stored frame
/// knows it is the identity, by its bits ([`Frame::motion`]); any other
/// frame is a [`Motion::Map`] whatever its value.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Motion<T: Real> {
    /// The bit-exact identity: composes with anything by returning it.
    Identity,
    /// Any other motion, as a general map.
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
}

#[cfg(test)]
mod tests {
    //! [`Frame`]'s exactness rule, at the stored-arrays-to-geometry
    //! doors — which the eval-level tests exercise only through whole
    //! documents.
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
}
