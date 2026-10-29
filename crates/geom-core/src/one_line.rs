//! **Two straight spans on one line**: where the projections of two
//! segments on one segment's axis overlap, or how far apart they lie.
//!
//! One reading, in the one crate every reader of two straight spans can
//! see, so each decides its own question on the same values and the
//! same charge; `geom_brep::curve_pair`'s router reads its gap here. The
//! question comes before any end-against-carrier reading: for any two
//! segments, collinear or not, a point `p` of one and `q` of the other
//! satisfy `|p − q| ≥ |(p − q)·û|`, so projections on a unit `û` that lie
//! apart by more than the band hold every pair of points apart by as
//! much.
//!
//! **The axis and its anchor.** The projections are read along the unit
//! `û` the caller passes (its carrier's or its chord's) from the first
//! span's START, never from a carrier's origin. A carrier's origin can
//! stand anywhere on its line, metres from either span; read from it, a
//! projection's own chain rounds at the origin's distance. The anchor
//! saves that only where the ends are given points: an end evaluated on
//! a far carrier carries the far metres already ("The charge").
//!
//! **The charge** covers two chains, and the reading is only as good as
//! the caller's account of the first.
//! - **How the ends were produced.** An end the caller evaluated, as
//!   `origin + dir·t` on a carrier, already carries that evaluation's
//!   rounding, which scales with the metres it was computed at
//!   (`|origin| + |dir|·|t|` for a line), not with where the end lies: a
//!   carrier whose origin stands `2²⁰` m from a span puts `~2⁻³³` m of
//!   rounding in an end at the origin of space. The caller passes those
//!   metres, summed over the four ends, as `produced`; zero only for ends
//!   that are given points, never computed.
//! - **The projections.** Read from `a0`, they round with the spans'
//!   own lengths, not with where the spans lie: the points are `f64`
//!   values as given, so their difference is one rounding of the real
//!   difference. With `u = 2⁻⁵³` and `p` any of `a1`, `b0`, `b1`, the
//!   chain `t(p) = fl(fl(p − a0)·û)` errs by at most
//!   - `u·|p − a0|` in the difference, carried through the dot;
//!   - `3u·|p − a0|` in the dot itself: three products and two sums in
//!     space (two products and one sum in the plane), with `|û| ≤ 1 + 5u`;
//!   - `5u·|p − a0|` for the unit: the caller's `û` is a rounded
//!     normalization, within `5u` of a unit in direction and length (the
//!     contract at [`on_one_line`](fn@on_one_line));
//!
//!   so `9u·|p − a0|` per projection, and the extent, two projections
//!   and one more difference, within `10u` of the two ends' distances
//!   from `a0`.
//!
//! [`crate::rounding_charge`] (sixteen half-ulps, so `16u` per metre) of
//! `produced` plus `|a1 − a0| + |b0 − a0| + |b1 − a0|` dominates both
//! chains, with room: every caller reads its margin past it. Charged at
//! the spans' own scale, two spans a kilometre out at `ε = 10⁻¹²` read as
//! finely as two at the origin; an end evaluated far away still carries
//! its evaluation through `produced`. A ROUNDING estimate, as that charge
//! is: at `Interval` the values are enclosures and the charge only
//! widens them.

use crate::{Point2, Point3, Real, Vec2, Vec3};

/// The overlap of two intervals on one line, and the rounding they
/// carry (module docs). The overlap's ends ([`Self::lo`], [`Self::hi`])
/// are read off the two intervals, so they cannot disagree with them.
#[derive(Clone, Copy, Debug)]
pub struct OnOneLine<T> {
    /// The rounding the ends carry ([`crate::rounding_charge`]).
    pub noise: T,
    /// The first interval as read, its ends in the order given.
    pub a: (T, T),
    /// The second interval as read, its ends in the order given: for
    /// [`on_one_line`](fn@on_one_line), `b`'s ends' projections
    /// `((b0 − a0)·û, (b1 − a0)·û)`, which a caller deciding which end
    /// bounds the overlap reads on their own.
    pub b: (T, T),
}

impl<T: Real> OnOneLine<T> {
    /// The overlap of the intervals `a` and `b`, each a pair of ends in
    /// either order, read by value, carrying `noise`.
    #[must_use]
    pub fn of(a: (T, T), b: (T, T), noise: T) -> Self {
        Self { noise, a, b }
    }

    /// The overlap's lower end: the greater of the two intervals' lower
    /// ends.
    #[must_use]
    pub fn lo(&self) -> T {
        self.a.0.min(self.a.1).max(self.b.0.min(self.b.1))
    }

    /// The overlap's upper end: the lesser of the two intervals' upper
    /// ends.
    #[must_use]
    pub fn hi(&self) -> T {
        self.a.0.max(self.a.1).min(self.b.0.max(self.b.1))
    }

    /// The overlap's signed length: positive an overlap, negative the
    /// gap between the intervals, negated.
    #[must_use]
    pub fn extent(&self) -> T {
        self.hi() - self.lo()
    }

    /// The overlap's middle (between the intervals where they are
    /// apart).
    #[must_use]
    pub fn mid(&self) -> T {
        (self.lo() + self.hi()) * T::from_f64(0.5)
    }

    /// The gap past the charge: positive where the intervals lie apart
    /// by more than their rounding, so no point of one comes within it
    /// of the other (module docs). A caller decides it on its own row.
    ///
    /// Where they overlap it is negative by the overlap's DEPTH, the
    /// lesser run from one interval's lower end to the other's upper
    /// end, not by its length: an interval that is a point (a span
    /// across the axis) inside the other reads how deep inside it lies,
    /// where its overlap's length is zero at every depth. Where the
    /// intervals are apart it is the overlap's negated length, bit for
    /// bit; where they overlap it is never above it, so a `Positive`
    /// reading never moves, but over an enclosure a value nearer the band
    /// can read in band where the length read a definite `Zero`, and a
    /// reader that keeps a screen's escalation sees the difference.
    #[must_use]
    pub fn apart(&self) -> T {
        self.gap() - self.noise
    }

    /// The signed gap between the intervals, before the charge
    /// ([`Self::apart`]).
    fn gap(&self) -> T {
        let (a_lo, a_hi) = (self.a.0.min(self.a.1), self.a.0.max(self.a.1));
        let (b_lo, b_hi) = (self.b.0.min(self.b.1), self.b.0.max(self.b.1));
        (b_lo - a_hi).max(a_lo - b_hi)
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
            produced: T,
        ) -> OnOneLine<T> {
            let along = |p: $point<T>| (p - a0).dot(unit);
            let terms = produced + (a1 - a0).norm() + (b0 - a0).norm() + (b1 - a0).norm();
            OnOneLine::of(
                (T::zero(), along(a1)),
                (along(b0), along(b1)),
                crate::rounding_charge(terms),
            )
        }
    };
}

on_one_line!(
    /// **The overlap of the segments `b0 b1` and `a0 a1` projected on the
    /// unit `unit`, read from `a0`** (module docs): `a`'s interval is
    /// `[0, (a1 − a0)·û]`, `b`'s its ends' projections. `produced` is the
    /// metres the four ends were computed at (module docs, "The charge").
    ///
    /// **The axis's contract.** `unit` is the caller's rounded
    /// normalization of a nonzero vector, `v·(1/|v|)` or `v.normalize()`,
    /// which holds it within `5u` of a unit in length and of `v` in
    /// direction; the charge's derivation counts on that (module docs).
    /// Every caller passes one: `geom_brep::curve_pair`'s router its
    /// carrier's direction, `topo`'s check 9 its edges' chords. An axis
    /// built any other way is outside the charge.
    on_one_line,
    Point3,
    Vec3
);

on_one_line!(
    /// [`on_one_line`](fn@on_one_line) in the plane, under the same axis
    /// contract: the reading for a planar reader of two straight spans,
    /// such as `profile::seg`'s segment pairs.
    on_one_line_2d,
    Point2,
    Vec2
);

#[cfg(test)]
mod rows {
    #![allow(clippy::unwrap_used)]

    use crate::{Bounds, Interval, Point3, Real, Vec3};
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

    /// `v` as an enclosure.
    fn iv_vec(v: Vec3<f64>) -> Vec3<Interval> {
        Vec3::new(
            Interval::from_f64(v.x),
            Interval::from_f64(v.y),
            Interval::from_f64(v.z),
        )
    }

    /// A unit direction drawn uniformly over the cube's directions.
    fn direction(rng: &mut fuzz::Rng) -> Vec3<f64> {
        Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        )
        .normalize()
    }

    /// **The charge is no looser than sixteen times what it stands for.**
    /// Containment alone grows easier as the charge grows, so a sweep also
    /// reads the largest share of the charge any draw spends: `0.16`–`0.34`
    /// over five runs of either sweep at the default effort, against this
    /// floor of `1/16`. A floor bounds the loosest the charge may be
    /// anywhere; that it is at the spans' own scale is
    /// [`the_charge_is_at_the_spans_scale_wherever_they_lie`]'s.
    fn tight(worst: f64) {
        assert!(
            worst >= 1.0 / 16.0,
            "no draw spent a sixteenth of its charge (worst {worst}): the \
             charge stands for far more rounding than the reading carries — {}",
            fuzz::replay()
        );
    }

    /// Each of `lo`, `hi`, the extent and the gap of `got` lies within its charge
    /// of the enclosure `real` of the same real quantity; returns the
    /// largest share of the charge any of them spends.
    fn holds(got: super::OnOneLine<f64>, real: super::OnOneLine<Interval>) -> f64 {
        let mut spent = 0.0_f64;
        for (what, v, e) in [
            ("lo", got.lo(), real.lo()),
            ("hi", got.hi(), real.hi()),
            ("extent", got.extent(), real.extent()),
            ("gap", got.gap(), real.gap()),
        ] {
            assert!(
                v - got.noise <= e.lo() && e.hi() <= v + got.noise,
                "{what}: {v} ± {} does not hold {e:?} — {}",
                got.noise,
                fuzz::replay()
            );
            spent = spent.max((v - e.lo()).abs().max((e.hi() - v).abs()) / got.noise);
        }
        spent
    }

    /// **The ends, the extent and the gap carry their rounding within the
    /// charge.** Against the same reading at `Interval`, an enclosure of
    /// the real projections of the same `f64` points, each of `lo`, `hi`,
    /// the extent and the gap lies within `noise` of every real value it stands for, over
    /// segments drawn at every scale and offset from the origin, half of
    /// them near each other wherever they lie (the charge's tightest
    /// regime: it is at the spans' own scale, not the origin's). Red where
    /// the charge is dropped, where it is more than sixteen times loose
    /// ([`tight`]), and where an overlap's gap reads shallower than its
    /// depth.
    #[test]
    fn the_overlap_lies_within_its_charge_of_the_real_one() {
        let mut rng = fuzz::start("one-line-charge");
        let mut worst = 0.0_f64;
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
            // Half the draws put the two spans near each other wherever
            // they lie, the regime the charge at the spans' own scale is
            // tightest in; half scatter them `far` apart.
            let a0 = at(&mut rng);
            let b0 = if rng.range(0.0, 1.0) < 0.5 {
                a0 + Vec3::new(
                    scale * rng.range(-1.0, 1.0),
                    scale * rng.range(-1.0, 1.0),
                    scale * rng.range(-1.0, 1.0),
                )
            } else {
                at(&mut rng)
            };
            let d = direction(&mut rng);
            let a1 = a0 + d * scale;
            let b1 = b0 + d * (scale * rng.range(-2.0, 2.0));
            let got = on_one_line(d, (a0, a1), (b0, b1), 0.0);
            let real = on_one_line(
                iv_vec(d),
                (iv(a0), iv(a1)),
                (iv(b0), iv(b1)),
                Interval::zero(),
            );
            worst = worst.max(holds(got, real));
            // Overlapping, the gap reads at least the overlap's depth,
            // the lesser run from one interval's lower end to the other's
            // upper end; read by length, a span inside the other reads
            // shallower than it lies.
            let ((al, ah), (bl, bh)) = (
                (got.a.0.min(got.a.1), got.a.0.max(got.a.1)),
                (got.b.0.min(got.b.1), got.b.0.max(got.b.1)),
            );
            let depth = (ah - bl).min(bh - al);
            if depth > 0.0 {
                assert!(
                    got.apart() <= -depth,
                    "overlapping by {depth}, the gap reads {} — {}",
                    got.apart(),
                    fuzz::replay()
                );
            }
        }
        tight(worst);
    }

    /// **An overlap reads its depth, not its length.** A span across the
    /// axis projects to a point; one metre inside a three-metre span its
    /// overlap is zero long, and the gap reads it a metre deep, so the
    /// pair reads definitely not apart rather than at the gap's zero.
    #[test]
    fn a_point_inside_the_other_span_reads_how_deep_it_lies() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let across = on_one_line(
            x,
            (Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0)),
            (Point3::new(1.0, 1.5, 0.0), Point3::new(1.0, 3.0, 0.0)),
            0.0,
        );
        assert_eq!(across.extent(), 0.0, "the overlap is a point");
        assert_eq!(
            across.apart(),
            -1.0 - across.noise,
            "the gap is the point's depth inside the span, past the charge"
        );
        let beyond = on_one_line(
            x,
            (Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0)),
            (Point3::new(4.0, 1.5, 0.0), Point3::new(4.0, 3.0, 0.0)),
            0.0,
        );
        assert_eq!(
            beyond.apart(),
            1.0 - beyond.noise,
            "apart, the gap is the distance between the spans"
        );
    }

    /// **The charge is at the spans' own scale, wherever they lie.** Two
    /// spans a metre long, a metre apart, `10³` … `10⁶` m from the origin:
    /// their `noise` is bounded by the charge of a few metres, the spans'
    /// own lengths from `a0`, at every distance from the origin. Red where
    /// the charge is read at the points' distances from the origin, the
    /// scale this replaced: there `noise` grows with `|p|`, about `10⁶`
    /// times this bound at the far end of the draw.
    #[test]
    fn the_charge_is_at_the_spans_scale_wherever_they_lie() {
        let mut rng = fuzz::start("one-line-span-scale");
        for _ in 0..fuzz::scaled(2_000) {
            let far = 10f64.powf(rng.range(3.0, 6.0));
            let a0 = Point3::new(
                far * rng.range(-1.0, 1.0),
                far * rng.range(-1.0, 1.0),
                far * rng.range(-1.0, 1.0),
            );
            let d = direction(&mut rng);
            let off = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ) * (1.0 / 3f64.sqrt());
            let (a1, b0) = (a0 + d, a0 + off);
            let b1 = b0 + d;
            let got = on_one_line(d, (a0, a1), (b0, b1), 0.0);
            // `|a1 − a0| + |b0 − a0| + |b1 − a0| ≤ 1 + 1 + 2` metres.
            let bound = crate::rounding_charge(4.0);
            assert!(
                got.noise <= bound,
                "{} m out: noise {:e} past the spans' own {:e} — {}",
                (a0 - Point3::origin()).norm(),
                got.noise,
                bound,
                fuzz::replay()
            );
        }
    }

    /// **Ends evaluated on far carriers carry their evaluation within the
    /// charge.** Each end is `origin + dir·t` in `f64` on a carrier whose
    /// origin stands up to `10⁶` m from its span, charged its
    /// `|origin| + |dir|·|t|` as `produced`. Against the same reading at
    /// `Interval` of the same carriers' exact ends, an enclosure of the
    /// real projections, each of `lo`, `hi` and the extent lies within
    /// `noise` of every real value it stands for, and spends enough of it
    /// ([`tight`]). Red where `produced` is dropped: the ends' own
    /// rounding, `~|origin|·2⁻⁵³`, is then charged only at their distance
    /// from the origin of space.
    #[test]
    fn ends_evaluated_on_far_carriers_lie_within_the_charge() {
        let mut rng = fuzz::start("one-line-produced");
        let mut worst = 0.0_f64;
        for _ in 0..fuzz::scaled(20_000) {
            let scale = 10f64.powf(rng.range(-3.0, 3.0));
            let far = 10f64.powf(rng.range(0.0, 6.0));
            let d = direction(&mut rng);
            // The spans lie near the origin of space, scattered by
            // `scale`; their carriers' origins lie `far` along `±d`.
            let carrier = |rng: &mut fuzz::Rng| {
                let back = far
                    * rng.range(0.5, 1.0)
                    * if rng.range(-1.0, 1.0) < 0.0 {
                        -1.0
                    } else {
                        1.0
                    };
                let off = Vec3::new(
                    scale * rng.range(-1.0, 1.0),
                    scale * rng.range(-1.0, 1.0),
                    scale * rng.range(-1.0, 1.0),
                );
                let origin = Point3::origin() + off - d * back;
                let t0 = back + scale * rng.range(-1.0, 1.0);
                (origin, (t0, t0 + scale * rng.range(-2.0, 2.0)))
            };
            let ((oa, (ta0, ta1)), (ob, (tb0, tb1))) = (carrier(&mut rng), carrier(&mut rng));
            let at = |o: Point3<f64>, t: f64| o + d * t;
            let produced = [(oa, ta0), (oa, ta1), (ob, tb0), (ob, tb1)]
                .into_iter()
                .map(|(o, t)| (o - Point3::origin()).norm() + d.norm() * t.abs())
                .sum::<f64>();
            let got = on_one_line(
                d,
                (at(oa, ta0), at(oa, ta1)),
                (at(ob, tb0), at(ob, tb1)),
                produced,
            );
            let d_iv = iv_vec(d);
            let real_at = |o: Point3<f64>, t: f64| iv(o) + d_iv * Interval::from_f64(t);
            let real = on_one_line(
                d_iv,
                (real_at(oa, ta0), real_at(oa, ta1)),
                (real_at(ob, tb0), real_at(ob, tb1)),
                Interval::zero(),
            );
            worst = worst.max(holds(got, real));
        }
        tight(worst);
    }
}
