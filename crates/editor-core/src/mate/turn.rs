//! **A turn onto a direction, and a direction across one**: what the
//! viewer's snap reads beside the value chart ([`super::value`]), whose
//! least turn, and this one, are geom-core's one decided mint
//! ([`RigidMotion::least_turn`]), so the turn a gesture writes and the
//! turn the chart reads it back by are the same arithmetic.

use geom_core::linalg::{Mat3, Point3, Vec3};
use geom_core::{Band, Lever, RigidMotion, UnitVec3};

use super::{SlideAxes, SolveScalar, TargetAxis};

/// The decision name of the snap's least turn ([`turn_onto`]).
pub const SNAP_TURN: &str = "mate_snap_turn";

/// **A turn carrying the unit `from` onto the unit `to`**: the least
/// turn ([`RigidMotion::least_turn`], over the session box's lever: a
/// gesture has no extent in hand), or where that refuses, in the band
/// about its pole, the half turn `H` about a direction across `from`
/// ([`across`]) followed by the least turn carrying `H·from = −from` onto
/// `to`. That second turn is near the identity (its chord is near 2), so
/// it is far from its own pole and well conditioned, and the composition
/// carries `from` onto `to` within a few ulps at every tilt in the band.
/// Its roll about `to` is `across(from)`'s, as the bare half turn's was
/// at the pole exactly. So a direction standing backwards, or nearly,
/// still seats on `to`.
#[must_use]
pub fn turn_onto(from: Vec3<f64>, to: Vec3<f64>, band: Band) -> Mat3<f64> {
    let unit = |v: Vec3<f64>| UnitVec3::new(v, SNAP_TURN, band).ok();
    let least = |a: Vec3<f64>, b: Vec3<f64>| {
        unit(a).zip(unit(b)).and_then(|(f, t)| {
            RigidMotion::least_turn(
                f,
                t,
                Point3::origin(),
                Lever::session_box(),
                SNAP_TURN,
                band,
            )
            .ok()
        })
    };
    if let Some(turn) = least(from, to) {
        return turn.linear();
    }
    let half = half_turn(across(from).normalize());
    match least(half * from, to) {
        Some(rest) => rest.linear() * half,
        None => half,
    }
}

/// A half turn about the unit `a`: `2 a aᵀ − I`.
fn half_turn(a: Vec3<f64>) -> Mat3<f64> {
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

#[cfg(test)]
mod tests {
    use super::{RigidMotion, turn_onto};
    use geom_core::linalg::{Point3, Vec3};
    use geom_core::tolerance::SESSION_BOX_EXTENT;
    use geom_core::{Band, Lever, Tol, UnitVec3};

    /// **Inside the least turn's band the snap still seats**: an oblique
    /// `to`, and `from` its opposite tilted by a fraction of the band's
    /// edge (the chord `2·box/R` at the run's ε: about 4e-4 at ε 1e-9,
    /// about 0.39 at ε 1e-12), where the mint refuses. `turn_onto` carries
    /// `from` onto `to`, the landing within ε at the session box's lever,
    /// and is a rotation. *Breaks if* the fallback is the bare half turn,
    /// which carries `from` onto `−from`, off `to` by about the tilt.
    #[test]
    fn inside_the_band_the_snap_carries_from_onto_to() {
        let band = Band::linear(Tol::witness()).expect("a band");
        let eps = Tol::witness().get().eps;
        let edge = 2.0 * SESSION_BOX_EXTENT / band.rounding_reach();
        let norm = |v: Vec3<f64>| v * (1.0 / v.norm());
        let to = norm(Vec3::new(0.3, -0.2, 0.9));
        let w = norm(to.cross(Vec3::new(1.0, 2.0, 3.0)));
        for frac in [0.5, 0.1, 1e-3] {
            let tau = (edge * frac).min(1.0);
            let from = -to * tau.cos() + w.cross(-to) * tau.sin();
            let unit = |v| UnitVec3::new(v, super::SNAP_TURN, band).unwrap();
            assert!(
                RigidMotion::least_turn(
                    unit(from),
                    unit(to),
                    Point3::origin(),
                    Lever::session_box(),
                    super::SNAP_TURN,
                    band
                )
                .is_err(),
                "tilt {tau:e} is inside the band, where the mint refuses"
            );
            let m = turn_onto(from, to, band);
            let landing = (m * from - to).norm();
            assert!(
                landing * SESSION_BOX_EXTENT <= eps,
                "tilt {tau:e}: |turn·from − to| = {landing:e}, past ε {eps:e} at the box's lever"
            );
            let g = m.transpose() * m;
            let id = geom_core::linalg::Mat3::<f64>::identity();
            let rigid = [g.c0 - id.c0, g.c1 - id.c1, g.c2 - id.c2]
                .into_iter()
                .map(Vec3::norm)
                .fold(0.0, f64::max);
            assert!(rigid < 1e-14, "tilt {tau:e}: not a rotation, {rigid:e}");
        }
    }
}
