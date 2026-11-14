//! **The least turn's pole, and a direction across one**: what the value
//! chart ([`super::value`]) and the viewer's snap both read beside the
//! least turn, whose arithmetic is geom-core's ([`LeastTurn`], its one
//! home), so the turn a gesture writes and the turn the chart reads it
//! back by are the same arithmetic.

use geom_core::linalg::{Mat3, Vec3};

use super::{SlideAxes, SolveScalar, TargetAxis};

pub use geom_core::LeastTurn;

/// **Whether `turn` is at its pole**, `from` against `to`, where it has
/// no answer: its `1 + c` not above zero, quoted. A reader's own verdict
/// about the band (the chart's, `Backwards`) is its own. The turn itself
/// is geom-core's ([`LeastTurn`]), its one home.
#[must_use]
pub fn at_pole<T: SolveScalar>(turn: &LeastTurn<T>) -> bool {
    turn.one_plus_c().quoted() <= 0.0
}

/// **A turn carrying the unit `from` onto the unit `to`**: the least
/// turn ([`LeastTurn`]), or at its pole a half turn about a direction
/// across `from` ([`across`]), so a direction standing backwards still
/// turns onto `to`.
#[must_use]
pub fn turn_onto(from: Vec3<f64>, to: Vec3<f64>) -> Mat3<f64> {
    let turn = LeastTurn::between(from, to);
    if !at_pole(&turn) {
        return turn.matrix();
    }
    let a = across(from).normalize();
    // A half turn about `a`: `2 a aᵀ − I`.
    let col = |e: Vec3<f64>| a * (2.0 * a.dot(e)) - e;
    Mat3::from_cols(
        col(Vec3::unit_x()),
        col(Vec3::unit_y()),
        col(Vec3::unit_z()),
    )
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

/// **The target axes the other slides of a bundle with no mate run
/// along, beside slides along `dirs`** (D10 `[placement.values]`): the
/// target axes that with `dirs` are independent — beside one direction
/// the two other than the one it runs most along, beside two the one
/// their plane's normal runs most along. What a gesture reads off a
/// picked direction before it writes a row along it
/// ([`super::Placed::unmated_across`]); the evaluation decides the
/// slides' independence again where the direction moves.
#[must_use]
pub fn slides_across(dirs: &[Vec3<f64>]) -> SlideAxes {
    let most = |v: Vec3<f64>| {
        let q = [v.x.abs(), v.y.abs(), v.z.abs()];
        (0..3).fold(0, |b, i| if q[i] > q[b] { i } else { b })
    };
    match dirs {
        [d] => {
            let m = most(*d);
            SlideAxes::of(TargetAxis::ALL.into_iter().filter(|a| a.index() != m))
        }
        [a, b] => SlideAxes::of([TargetAxis::of_index(most(a.cross(*b)))]),
        _ => SlideAxes::default(),
    }
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
