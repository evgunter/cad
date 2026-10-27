//! **The least turn, and a direction across one**: the one home of the
//! two constructions the value chart ([`super::value`]) and the viewer's
//! snap both read, so the turn a gesture writes and the turn the chart
//! reads it back by are the same arithmetic.

use geom_core::linalg::{Mat3, Vec3};

use super::{SlideAxes, SolveScalar, TargetAxis};

/// **The least turn carrying the unit `from` onto the unit `to`**,
/// `S = c·I + [k]× + k kᵀ / (1 + c)` with `c = from · to` and
/// `k = from × to`: the turn about `k` by the angle between them.
///
/// It has no answer at its one pole, `from = −to`, where `1 + c` is
/// zero; a reader decides the pole from [`Self::one_plus_c`] before it
/// applies the turn.
#[derive(Debug, Clone, Copy)]
pub struct LeastTurn<T: geom_core::Real> {
    c: T,
    k: Vec3<T>,
    one_plus_c: T,
}

impl<T: SolveScalar> LeastTurn<T> {
    /// The least turn carrying `from` onto `to`.
    #[must_use]
    pub fn between(from: Vec3<T>, to: Vec3<T>) -> Self {
        let one = T::one();
        let k = from.cross(to);
        let c = from.dot(to);
        // Which formula is a choice of arithmetic, not a verdict: the two
        // are one number, and the first loses no digit near the pole.
        let one_plus_c = if c.quoted() < 0.0 {
            k.dot(k) / (one - c)
        } else {
            one + c
        };
        Self { c, k, one_plus_c }
    }

    /// `1 + c`: zero at the pole, two where the directions agree.
    #[must_use]
    pub fn one_plus_c(&self) -> T {
        self.one_plus_c
    }

    /// `S·v`.
    #[must_use]
    pub fn apply(&self, v: Vec3<T>) -> Vec3<T> {
        v * self.c + self.k.cross(v) + self.k * (self.k.dot(v) / self.one_plus_c)
    }

    /// `S`, column by column.
    #[must_use]
    pub fn matrix(&self) -> Mat3<T> {
        Mat3::from_cols(
            self.apply(Vec3::unit_x()),
            self.apply(Vec3::unit_y()),
            self.apply(Vec3::unit_z()),
        )
    }
}

/// **Whether `x` is zero exactly**: a point zero at every lane, read
/// off its value channel at a dual's.
pub(crate) fn zero<T: SolveScalar>(x: T) -> bool {
    x.exactly_zero() || (x.quoted() == 0.0 && x.upper() == 0.0)
}

/// **The target axis `n` lies along exactly**, if it lies along one: its
/// index.
#[must_use]
pub fn aligned<T: SolveScalar>(n: Vec3<T>) -> Option<usize> {
    match [n.x, n.y, n.z].map(zero) {
        [false, true, true] => Some(0),
        [true, false, true] => Some(1),
        [true, true, false] => Some(2),
        _ => None,
    }
}

/// **The slides a mate leaving `count` of them leaves**, given `n`,
/// where the mate's orientation puts the copy's mated direction among
/// the target axes: all three beside a lone direction; beside a plane,
/// the two other than the axis `n` lies most along, whose slide the
/// plane fixes; beside an axis, that one, which it leaves. Where `n`
/// lies on an axis exactly that is the axis; otherwise the greatest
/// component wins, a tie going to the earlier axis. The pair it names
/// beside a plane spans what the plane leaves, as the chart's span check
/// reads it, wherever any pair does.
#[must_use]
pub fn slides_left_along<T: SolveScalar>(count: usize, n: Vec3<T>) -> SlideAxes {
    let q = [n.x.quoted().abs(), n.y.quoted().abs(), n.z.quoted().abs()];
    let most = aligned(n).unwrap_or_else(|| (0..3).fold(0, |b, i| if q[i] > q[b] { i } else { b }));
    SlideAxes::of(TargetAxis::ALL.into_iter().filter(|a| match count {
        3 => true,
        2 => a.index() != most,
        1 => a.index() == most,
        _ => false,
    }))
}

/// **A direction across the unit `n`**, the reference a spin about `n`
/// is measured from: where `n` lies along axis `j`, the next axis
/// `(j + 1) mod 3` exactly; otherwise the target axis least along `n`
/// with its part along `n` taken off, which the reader normalizes in its
/// own band.
#[must_use]
pub fn across<T: SolveScalar>(n: Vec3<T>) -> Vec3<T> {
    let axes = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
    if let Some(j) = aligned(n) {
        return axes[(j + 1) % 3];
    }
    let q = [n.x.quoted().abs(), n.y.quoted().abs(), n.z.quoted().abs()];
    let least = (0..3).fold(0, |b, i| if q[i] < q[b] { i } else { b });
    let e = axes[least];
    e - n * n.dot(e)
}
