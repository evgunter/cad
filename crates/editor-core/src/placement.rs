//! Cluster placement frames (ASSEMBLY-DESIGN A11; ASM-2A D-2).
//!
//! A11 puts placement on the **cluster**, never on the instance: a
//! placement cluster is a connected component of the instance–mate
//! graph, and its one frame places its gauge instance. In the mate-less
//! v1 every instance is a SINGLETON cluster, so [`crate::Doc`]'s
//! placement registry is keyed by the instantiate node's own id — the
//! key generalizes to a cluster representative when mates land (R2),
//! and nothing about the stored frame changes when it does.
//!
//! A missing entry is the IDENTITY frame: a legal, complete state, not
//! a hole. Zero-anchor and multi-anchor states are unrepresentable
//! because the registry holds at most one frame per cluster.

use geom_core::predicate::Band;
use geom_core::{Affine3, Mat3, Real, Vec3};

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
/// Rigidity is not a field invariant — it is a decided predicate, and
/// the kernel's placement door
/// ([`topo::transform_rigid`]) owns it. What the
/// edit door checks is the sign (see [`Frame::determinant`]).
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
/// makes the split/inline round trip exact — the frames a split hoists
/// or leaves behind compose back with zero arithmetic, so D-4's
/// bit-level volume identity never meets a rounding step.
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
    /// The identity placement — what a missing registry entry means.
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
    /// is what carries it through the `SetPlacement` door.
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

    /// The composition `self ∘ inner`: the frame that places by
    /// `inner` first, then by `self` — inline's rule (ASM-4 D-3: the
    /// instance's cluster frame composed onto the part's placements).
    ///
    /// A bit-exact identity on either side returns the other operand
    /// verbatim; the general product is [`Affine3`]'s own
    /// multiplication at `f64` ([`Frame`]'s exactness rule).
    #[must_use]
    pub fn compose(&self, inner: &Frame) -> Frame {
        if self.is_identity_bits() {
            return *inner;
        }
        if inner.is_identity_bits() {
            return *self;
        }
        Frame::from_affine(self.affine_f64() * inner.affine_f64())
    }

    /// Whether this frame is the stored identity, BY BITS — D-3's
    /// admission test for the identity fast path ([`Frame`]'s exactness
    /// rule), which skips the kernel map.
    #[must_use]
    pub fn is_identity_bits(&self) -> bool {
        self.bit_eq(&Self::IDENTITY)
    }

    /// **The frame half of A11/A6's admission rule, stated once**: a
    /// frame the document admits carries only numbers, and preserves
    /// orientation.
    ///
    /// One predicate with one home, asked wherever a document admits a
    /// frame — the A11 cluster registry ([`crate::doc::PlacementFault`])
    /// and a placement rule's listed frames
    /// ([`crate::node::PlacementRuleFault`]) — so the two cannot come to
    /// hold frames to different standards. Each caller keeps its own
    /// arms, says WHICH frame is at fault in its own vocabulary, and
    /// forwards this answer's sentence rather than restating it.
    ///
    /// It lives on [`Frame`] because the rule is about a frame and
    /// nothing else: it reads no document, no node and no registry.
    #[must_use]
    pub(crate) fn admission_fault(&self) -> Option<FrameFault> {
        if !self.is_finite() {
            return Some(FrameFault::NonFinite);
        }
        let determinant = self.determinant();
        (determinant <= 0.0).then_some(FrameFault::Improper { determinant })
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
/// every door that admits a frame. Public because the load door
/// carries it out on [`crate::PersistError::MaintenanceFrame`], for a
/// recorded maintenance row held to the same rule.
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
}

// The ONE prose for the frame rule: a predicate clause, so each door
// forwards it into its own subject ("placement 2 …", "the placement
// frame on node 7 …") rather than inventing a second wording for the
// fact they share.
impl core::fmt::Display for FrameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite => f.write_str("carries a non-finite coordinate"),
            Self::Improper { determinant } => {
                write!(f, "is improper (mirroring): determinant {determinant}")
            }
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    //! [`Frame`]'s exactness rule, at the stored-arrays-to-geometry
    //! doors — which the eval-level tests exercise only through whole
    //! documents.

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
    /// linear-part snap is the one that silently changes an answer:
    /// `mate::solve`'s `reconcile` branches on
    /// `relative.is_identity_bits()` and its `true` arm DISCARDS the
    /// solved relative pose, so a gauge that rotated by a hair would
    /// read as "did not move".
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
