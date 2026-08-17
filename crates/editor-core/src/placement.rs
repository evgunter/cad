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

use geom_core::{Affine3, Mat3, Real, Vec3};

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

impl Frame {
    /// The identity placement — what a missing registry entry means.
    pub const IDENTITY: Self = Self {
        columns: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };

    /// The pure translation by `v`.
    pub fn translation(v: [f64; 3]) -> Self {
        Self {
            translation: v,
            ..Self::IDENTITY
        }
    }

    /// The rotation by `angle` radians (right-hand rule) about the axis
    /// through the origin with direction `axis`, then translation by
    /// `v` — the `Transform` node's own composition order (D9), so a
    /// placement and a modeled transform of the same part agree BIT FOR
    /// BIT.
    ///
    /// The agreement is what makes the claim testable, and it is why
    /// the axis is normalized HERE: the transform node normalizes its
    /// axis before building the rotation (`eval::wire`'s `unit`), and
    /// `Mat3::rotation_about` normalizes again internally, so an
    /// un-normalized axis would take one fewer rounding step through
    /// this door than through that one. Same input, same expression,
    /// same bits — for any axis, not just unit ones.
    ///
    /// A zero (or non-finite) axis normalizes to NaN and yields a
    /// non-finite frame, which `SetPlacement` refuses typed
    /// ([`Frame::is_finite`]) — the transform node's own
    /// `DegenerateDirection` refusal, arriving at this layer's door.
    pub fn rotate_then_translate(axis: [f64; 3], angle: f64, v: [f64; 3]) -> Self {
        let m = Mat3::rotation_about(Vec3::new(axis[0], axis[1], axis[2]).normalize(), angle);
        Self {
            columns: [
                [m.c0.x, m.c0.y, m.c0.z],
                [m.c1.x, m.c1.y, m.c1.z],
                [m.c2.x, m.c2.y, m.c2.z],
            ],
            translation: v,
        }
    }

    /// The frame denoting an affine map — the inverse of
    /// [`Frame::affine`], for the mate solve's poses coming back from
    /// the coset algebra (ASM-R2a D-5).
    ///
    /// A BIT-exact identity map returns [`Frame::IDENTITY`] verbatim,
    /// the same rule [`Frame::compose`] follows: an unmated instance's
    /// solved relative pose must land on the identity's own bits, or
    /// the mate-less document's evaluation would differ from the
    /// pre-mate one by a rounding step that never happened.
    pub fn from_affine(a: geom_core::Affine3<f64>) -> Self {
        let out = Self {
            columns: [
                [a.linear.c0.x, a.linear.c0.y, a.linear.c0.z],
                [a.linear.c1.x, a.linear.c1.y, a.linear.c1.z],
                [a.linear.c2.x, a.linear.c2.y, a.linear.c2.z],
            ],
            translation: [a.translation.x, a.translation.y, a.translation.z],
        };
        if out.is_identity_bits() {
            Self::IDENTITY
        } else {
            out
        }
    }

    /// Whether every stored coordinate is finite.
    pub fn is_finite(&self) -> bool {
        self.columns
            .iter()
            .flatten()
            .chain(self.translation.iter())
            .all(|x| x.is_finite())
    }

    /// The linear part's determinant, in [`Mat3::determinant`]'s exact
    /// evaluation order (D9): a proper frame's is `+1`, an improper
    /// (mirroring) frame's `−1`.
    pub fn determinant(&self) -> f64 {
        self.linear::<f64>().determinant()
    }

    /// The linear part, in the backend scalar.
    pub fn linear<T: Real>(&self) -> Mat3<T> {
        let col = |c: [f64; 3]| Vec3::new(T::from_f64(c[0]), T::from_f64(c[1]), T::from_f64(c[2]));
        Mat3::from_cols(
            col(self.columns[0]),
            col(self.columns[1]),
            col(self.columns[2]),
        )
    }

    /// The affine map this frame denotes, in the backend scalar — what
    /// the kernel's placement door consumes.
    pub fn affine<T: Real>(&self) -> Affine3<T> {
        Affine3::from_parts(
            self.linear::<T>(),
            Vec3::new(
                T::from_f64(self.translation[0]),
                T::from_f64(self.translation[1]),
                T::from_f64(self.translation[2]),
            ),
        )
    }

    /// The composition `self ∘ inner`: the frame that places by
    /// `inner` first, then by `self` — inline's rule (ASM-4 D-3: the
    /// instance's cluster frame composed onto the part's placements).
    ///
    /// A BIT-exact identity on either side returns the other operand
    /// VERBATIM — the placing door's own fast-path rule, and what
    /// makes the split/inline round trip exact: the frames a split
    /// hoists or leaves behind compose back with zero arithmetic, so
    /// D-4's bit-level volume identity never meets a rounding step.
    /// The general product is plain f64 matrix arithmetic in a fixed
    /// order (D9-deterministic, not claimed exact).
    pub fn compose(&self, inner: &Frame) -> Frame {
        if self.is_identity_bits() {
            return *inner;
        }
        if inner.is_identity_bits() {
            return *self;
        }
        let l = self.linear::<f64>() * inner.linear::<f64>();
        let t = self.linear::<f64>()
            * Vec3::new(
                inner.translation[0],
                inner.translation[1],
                inner.translation[2],
            );
        Frame {
            columns: [
                [l.c0.x, l.c0.y, l.c0.z],
                [l.c1.x, l.c1.y, l.c1.z],
                [l.c2.x, l.c2.y, l.c2.z],
            ],
            translation: [
                t.x + self.translation[0],
                t.y + self.translation[1],
                t.z + self.translation[2],
            ],
        }
    }

    /// Whether this frame is the stored identity, BY BITS — the
    /// identity fast-path's admission test (D-3: an identity placement
    /// skips the kernel map, and only a bit-exact identity may, since
    /// any other value could round).
    pub fn is_identity_bits(&self) -> bool {
        self.bit_eq(&Self::IDENTITY)
    }

    /// Bit-semantic frame equality (D7's comparator family): every
    /// coordinate compares by BITS, so `±0.0` do not conflate.
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

impl Default for Frame {
    fn default() -> Self {
        Self::IDENTITY
    }
}
