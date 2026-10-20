//! **Two straight spans on one line**: where the projections of two
//! segments on one segment's axis overlap, or how far apart they lie.
//!
//! One reading for every caller that asks it (`geom_brep::curve_pair`'s
//! router, `topo`'s census and check 9, `profile`'s segment pairs), so
//! each decides its own question on the same values and the same
//! charge. The question comes before any end-against-carrier reading:
//! for any two segments, collinear or not, a point `p` of one and `q` of
//! the other satisfy `|p − q| ≥ |(p − q)·û|`, so projections on a unit
//! `û` that lie apart by more than the band hold every pair of points
//! apart by as much.
//!
//! **The axis and its anchor.** The projections are read along the unit
//! `û` the caller passes (its carrier's or its chord's) from the first
//! span's START, never from a carrier's origin. A carrier's origin can
//! stand anywhere on its line, metres from either span; read from it, a
//! projection carries the origin's distance in its rounding.
//!
//! **The charge.** Each projection is a difference of two points and a
//! dot product with `û`, a chain of a few roundings each within half an
//! ulp of the points' coordinates; the extent is one more difference.
//! [`geom_core::rounding_charge`] of the four points' distances from the
//! origin dominates that chain (the same charge the harmonics' short
//! chains carry), and every caller reads its margin past it. A ROUNDING
//! estimate, as that charge is: at `Interval` the values are enclosures
//! and the charge only widens them.

use geom_core::{Point2, Point3, Real, Vec2, Vec3};

/// The overlap of two intervals on one line, and the rounding they
/// carry (module docs).
#[derive(Clone, Copy, Debug)]
pub struct OnOneLine<T> {
    /// The overlap's lower end: the greater of the two intervals' lower
    /// ends.
    pub lo: T,
    /// The overlap's upper end: the lesser of the two intervals' upper
    /// ends.
    pub hi: T,
    /// The rounding the ends carry ([`geom_core::rounding_charge`]).
    pub noise: T,
}

impl<T: Real> OnOneLine<T> {
    /// The overlap of the intervals `a` and `b`, each a pair of ends in
    /// either order, read by value, carrying `noise`.
    #[must_use]
    pub fn of(a: (T, T), b: (T, T), noise: T) -> Self {
        Self {
            lo: a.0.min(a.1).max(b.0.min(b.1)),
            hi: a.0.max(a.1).min(b.0.max(b.1)),
            noise,
        }
    }

    /// The overlap's signed length: positive an overlap, negative the
    /// gap between the intervals, negated.
    #[must_use]
    pub fn extent(&self) -> T {
        self.hi - self.lo
    }

    /// The overlap's middle (between the intervals where they are
    /// apart).
    #[must_use]
    pub fn mid(&self) -> T {
        (self.lo + self.hi) * T::from_f64(0.5)
    }

    /// The gap past the charge: positive where the intervals lie apart
    /// by more than their rounding, so no point of one comes within it
    /// of the other (module docs). A caller decides it on its own row.
    #[must_use]
    pub fn apart(&self) -> T {
        T::zero() - self.extent() - self.noise
    }
}

macro_rules! on_one_line {
    ($(#[$doc:meta])* $name:ident, $point:ident, $vec:ident) => {
        $(#[$doc])*
        #[must_use]
        pub fn $name<T: Real>(
            unit: $vec<T>,
            (a0, a1): ($point<T>, $point<T>),
            (b0, b1): ($point<T>, $point<T>),
        ) -> OnOneLine<T> {
            let along = |p: $point<T>| (p - a0).dot(unit);
            let o = $point::origin();
            let terms = (a0 - o).norm() + (a1 - o).norm() + (b0 - o).norm() + (b1 - o).norm();
            OnOneLine::of(
                (T::zero(), along(a1)),
                (along(b0), along(b1)),
                geom_core::rounding_charge(terms),
            )
        }
    };
}

on_one_line!(
    /// **The overlap of the segments `b0 b1` and `a0 a1` projected on the
    /// unit `unit`, read from `a0`** (module docs): `a`'s interval is
    /// `[0, (a1 − a0)·û]`, `b`'s its ends' projections.
    on_one_line,
    Point3,
    Vec3
);

on_one_line!(
    /// [`on_one_line`] in the plane.
    on_one_line_2d,
    Point2,
    Vec2
);

#[cfg(test)]
mod rows {
    #![allow(clippy::unwrap_used)]

    use geom_core::{Bounds, Interval, Point3, Real, Vec3};
    use test_utils::fuzz;

    use super::on_one_line;

    /// `v` as an enclosure.
    fn iv(v: Point3<f64>) -> Point3<Interval> {
        Point3::new(
            Interval::from_f64(v.x),
            Interval::from_f64(v.y),
            Interval::from_f64(v.z),
        )
    }

    /// **The ends and the extent carry their rounding within the charge.**
    /// Against the same reading at `Interval`, an enclosure of the real
    /// projections of the same `f64` points, each of `lo`, `hi` and the
    /// extent lies within `noise` of every real value it stands for, over
    /// segments drawn at every scale and offset from the origin. Red where
    /// the charge is dropped.
    #[test]
    fn the_overlap_lies_within_its_charge_of_the_real_one() {
        let mut rng = fuzz::start("one-line-charge");
        for _ in 0..fuzz::scaled(20_000) {
            let scale = 10f64.powf(rng.range(-3.0, 4.0));
            let far = 10f64.powf(rng.range(-3.0, 5.0));
            let at = |rng: &mut fuzz::Rng| {
                Point3::new(
                    far * rng.range(-1.0, 1.0) + scale * rng.range(-1.0, 1.0),
                    far * rng.range(-1.0, 1.0) + scale * rng.range(-1.0, 1.0),
                    far * rng.range(-1.0, 1.0) + scale * rng.range(-1.0, 1.0),
                )
            };
            let (a0, b0) = (at(&mut rng), at(&mut rng));
            let d = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            )
            .normalize();
            let a1 = a0 + d * scale;
            let b1 = b0 + d * (scale * rng.range(-2.0, 2.0));
            let got = on_one_line(d, (a0, a1), (b0, b1));
            let d_iv = Vec3::new(
                Interval::from_f64(d.x),
                Interval::from_f64(d.y),
                Interval::from_f64(d.z),
            );
            let real = on_one_line(d_iv, (iv(a0), iv(a1)), (iv(b0), iv(b1)));
            for (what, v, e) in [
                ("lo", got.lo, real.lo),
                ("hi", got.hi, real.hi),
                ("extent", got.extent(), real.extent()),
            ] {
                assert!(
                    v - got.noise <= e.lo() && e.hi() <= v + got.noise,
                    "{what}: {v} ± {} does not hold {e:?} — {}",
                    got.noise,
                    fuzz::replay()
                );
            }
        }
    }
}
