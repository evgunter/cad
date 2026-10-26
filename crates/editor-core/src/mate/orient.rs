//! **The orientation** (D10 `[placement.values]`, `[variables.kinds]`):
//! one of the 24 ways a source's axes lie along its target's, a
//! discrete kind a mate fixing a direction carries. It says where the
//! copy's mated direction should point; the value chart's least turn
//! carries that onto the mated direction ([`super::value`]).

use geom_core::Real;
use geom_core::linalg::{Mat3, Vec3};

/// **An axis of a space with its sense**: `+x`, `−z`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum SignedAxis {
    /// `+x`.
    #[serde(rename = "+x")]
    PlusX,
    /// `−x`.
    #[serde(rename = "-x")]
    MinusX,
    /// `+y`.
    #[serde(rename = "+y")]
    PlusY,
    /// `−y`.
    #[serde(rename = "-y")]
    MinusY,
    /// `+z`.
    #[serde(rename = "+z")]
    PlusZ,
    /// `−z`.
    #[serde(rename = "-z")]
    MinusZ,
}

impl SignedAxis {
    /// The six, in order.
    pub const ALL: [Self; 6] = [
        Self::PlusX,
        Self::MinusX,
        Self::PlusY,
        Self::MinusY,
        Self::PlusZ,
        Self::MinusZ,
    ];

    /// Which axis, `0` for `x`.
    #[must_use]
    pub fn index(self) -> usize {
        match self {
            Self::PlusX | Self::MinusX => 0,
            Self::PlusY | Self::MinusY => 1,
            Self::PlusZ | Self::MinusZ => 2,
        }
    }

    /// Whether its sense is negative.
    #[must_use]
    pub fn negative(self) -> bool {
        matches!(self, Self::MinusX | Self::MinusY | Self::MinusZ)
    }

    /// The axis of `index` with the sense `negative`.
    fn of(index: usize, negative: bool) -> Self {
        match (index, negative) {
            (0, false) => Self::PlusX,
            (0, true) => Self::MinusX,
            (1, false) => Self::PlusY,
            (1, true) => Self::MinusY,
            (2, false) => Self::PlusZ,
            _ => Self::MinusZ,
        }
    }

    /// The other sense.
    #[must_use]
    pub fn reversed(self) -> Self {
        Self::of(self.index(), !self.negative())
    }

    /// Its name: `+x`.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::PlusX => "+x",
            Self::MinusX => "-x",
            Self::PlusY => "+y",
            Self::MinusY => "-y",
            Self::PlusZ => "+z",
            Self::MinusZ => "-z",
        }
    }

    /// The unit vector, exactly.
    fn unit<T: Real>(self) -> Vec3<T> {
        let mut v = [T::zero(); 3];
        v[self.index()] = if self.negative() { -T::one() } else { T::one() };
        Vec3::new(v[0], v[1], v[2])
    }

    /// `self × other`, for two axes of different letters.
    fn cross(self, other: Self) -> Self {
        let (a, b) = (self.index(), other.index());
        let c = 3 - a - b;
        // `e_a × e_b` is `+e_c` where (a, b, c) is cyclic.
        let cyclic = (a + 1) % 3 == b;
        Self::of(c, cyclic == (self.negative() != other.negative()))
    }
}

/// **The orientation**: where the source's `x` and `z` lie among the
/// target's axes (`x: +x, z: −z`); its `y` is `z × x`. The identity
/// unless written. Every one of the 24 is a proper rotation, and the
/// two axes are of different letters by construction ([`Self::new`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "OrientationWire", into = "OrientationWire")]
pub struct Orientation {
    x: SignedAxis,
    z: SignedAxis,
}

/// The orientation as written on the wire.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OrientationWire {
    x: SignedAxis,
    z: SignedAxis,
}

impl TryFrom<OrientationWire> for Orientation {
    type Error = String;

    fn try_from(w: OrientationWire) -> Result<Self, Self::Error> {
        Self::new(w.x, w.z).ok_or_else(|| {
            format!(
                "an orientation's x and z lie along one axis ({} and {})",
                w.x.name(),
                w.z.name()
            )
        })
    }
}

impl From<Orientation> for OrientationWire {
    fn from(o: Orientation) -> Self {
        Self { x: o.x, z: o.z }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Orientation {
    /// The source's axes along the target's own.
    pub const IDENTITY: Self = Self {
        x: SignedAxis::PlusX,
        z: SignedAxis::PlusZ,
    };

    /// The orientation putting the source's `x` along `x` and its `z`
    /// along `z`; `None` where the two are of one letter.
    #[must_use]
    pub fn new(x: SignedAxis, z: SignedAxis) -> Option<Self> {
        (x.index() != z.index()).then_some(Self { x, z })
    }

    /// Where the source's `x` lies.
    #[must_use]
    pub fn x(self) -> SignedAxis {
        self.x
    }

    /// Where the source's `y` lies: `z × x`.
    #[must_use]
    pub fn y(self) -> SignedAxis {
        self.z.cross(self.x)
    }

    /// Where the source's `z` lies.
    #[must_use]
    pub fn z(self) -> SignedAxis {
        self.z
    }

    /// The 24, the identity first.
    #[must_use]
    pub fn all() -> Vec<Self> {
        let order = [
            SignedAxis::PlusZ,
            SignedAxis::MinusZ,
            SignedAxis::PlusX,
            SignedAxis::MinusX,
            SignedAxis::PlusY,
            SignedAxis::MinusY,
        ];
        SignedAxis::ALL
            .into_iter()
            .flat_map(|x| order.into_iter().filter_map(move |z| Self::new(x, z)))
            .collect()
    }

    /// Where the source's axis `index` lies.
    fn column(self, index: usize) -> SignedAxis {
        match index {
            0 => self.x,
            1 => self.y(),
            _ => self.z,
        }
    }

    /// **`v`, in source coordinates, carried into the target's**: each
    /// coordinate moved to its axis, its sign changed where the axis is
    /// reversed — exact, no arithmetic but a negation.
    #[must_use]
    pub fn apply<T: Real>(self, v: Vec3<T>) -> Vec3<T> {
        let from = [v.x, v.y, v.z];
        let mut to = [T::zero(); 3];
        for (k, &c) in from.iter().enumerate() {
            let axis = self.column(k);
            to[axis.index()] = if axis.negative() { -c } else { c };
        }
        Vec3::new(to[0], to[1], to[2])
    }

    /// **`v`, in target coordinates, carried back into the source's**:
    /// [`Self::apply`]'s inverse, exact.
    #[must_use]
    pub fn unapply<T: Real>(self, v: Vec3<T>) -> Vec3<T> {
        let from = [v.x, v.y, v.z];
        let mut to = [T::zero(); 3];
        for (k, slot) in to.iter_mut().enumerate() {
            let axis = self.column(k);
            let c = from[axis.index()];
            *slot = if axis.negative() { -c } else { c };
        }
        Vec3::new(to[0], to[1], to[2])
    }

    /// The rotation, its columns the source's axes in the target's.
    #[must_use]
    pub fn matrix<T: Real>(self) -> Mat3<T> {
        Mat3::from_cols(self.x.unit(), self.y().unit(), self.z.unit())
    }

    /// **The orientation nearest the rotation `r`** (a gesture's
    /// authoring of where a copy stands): the one whose axes lie
    /// closest to `r`'s columns, its trace against `r` greatest; the
    /// first in [`Self::all`]'s order among equals.
    #[must_use]
    pub fn nearest(r: &Mat3<f64>) -> Self {
        let cols = [r.c0, r.c1, r.c2];
        let score = |o: Self| -> f64 {
            (0..3)
                .map(|k| {
                    let a = o.column(k);
                    let c = cols[k];
                    let v = [c.x, c.y, c.z][a.index()];
                    if a.negative() { -v } else { v }
                })
                .sum()
        };
        let mut best = Self::IDENTITY;
        let mut top = f64::NEG_INFINITY;
        for o in Self::all() {
            let s = score(o);
            if s > top {
                best = o;
                top = s;
            }
        }
        best
    }

    /// **The orientation nearest `self` that puts the source direction
    /// `from` closest to the target direction `to`** — the one a refusal
    /// at the pole names: the greatest `(O·from)·to`, then the nearest
    /// to `self`, then one keeping `self`'s `x` where it is, then its
    /// `y`, then the first in [`Self::all`]'s order.
    #[must_use]
    pub fn toward(self, from: Vec3<f64>, to: Vec3<f64>) -> Self {
        let own = self.matrix::<f64>();
        let mut best = self;
        let mut top = (f64::NEG_INFINITY, f64::NEG_INFINITY, false, false);
        for o in Self::all() {
            let m = o.matrix::<f64>();
            let near = m.c0.dot(own.c0) + m.c1.dot(own.c1) + m.c2.dot(own.c2);
            let s = (
                o.apply(from).dot(to),
                near,
                o.x == self.x,
                o.y() == self.y(),
            );
            if s > top {
                best = o;
                top = s;
            }
        }
        best
    }
}

impl core::fmt::Display for Orientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "x: {}, z: {}", self.x.name(), self.z.name())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    /// **A tie keeps the author's `x`.** The upended `(x: +y, z: −z)`
    /// turned toward `+z`: every quarter turn about `z` puts `z` there
    /// and is equally near, and the one kept is the one leaving the
    /// author's `x` on `+y`. Breaks if the tie falls to [`Orientation::all`]'s
    /// order, which names the identity.
    #[test]
    fn a_tie_keeps_the_authors_x() {
        let upended = Orientation::new(SignedAxis::PlusY, SignedAxis::MinusZ)
            .expect("y and z lie along two axes");
        let z = Vec3::new(0.0, 0.0, 1.0);
        let got = upended.toward(z, z);
        assert_eq!(
            got,
            Orientation::new(SignedAxis::PlusY, SignedAxis::PlusZ).expect("y and z")
        );
    }

    /// Every orientation is a proper rotation whose `apply` is its
    /// matrix and whose `unapply` undoes it. Breaks if `cross` takes a
    /// wrong sign (a reflection would have determinant −1).
    #[test]
    fn the_24_are_rotations() {
        let all = Orientation::all();
        assert_eq!(all.len(), 24, "24 orientations");
        assert_eq!(all[0], Orientation::IDENTITY, "the identity first");
        let v = Vec3::new(1.0, 2.0, 3.0);
        for o in all {
            let m = o.matrix::<f64>();
            let t = |v: Vec3<f64>| (v.x, v.y, v.z);
            assert_eq!(t(m.c0.cross(m.c1)), t(m.c2), "{o}: right-handed");
            assert_eq!(t(m * v), t(o.apply(v)), "{o}: apply is the matrix");
            assert_eq!(t(o.unapply(o.apply(v))), t(v), "{o}: unapply undoes apply");
            assert_eq!(Orientation::nearest(&m), o, "{o}: nearest to itself");
        }
    }

    /// The wire refuses two axes of one letter. Breaks if `try_from`
    /// lets one through.
    #[test]
    fn the_wire_refuses_a_degenerate_orientation() {
        let ok: Orientation = serde_json::from_str(r#"{"x":"+y","z":"-z"}"#).expect("loads");
        assert_eq!(ok.y(), SignedAxis::PlusX, "z × x = −z × +y = +x");
        assert!(serde_json::from_str::<Orientation>(r#"{"x":"+x","z":"-x"}"#).is_err());
        assert_eq!(
            serde_json::to_string(&ok).expect("saves"),
            r#"{"x":"+y","z":"-z"}"#
        );
    }
}
