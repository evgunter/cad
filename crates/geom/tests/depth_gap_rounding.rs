//! The depth gap of two nested tori about the outer's core
//! (`Cover::range_about`), read at `f64` and at `Interval` against the
//! true gap: the reading a face pair's depth gap takes
//! (`topo::census::depth_gap`), and how its `f64` rounding falls.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Frame;
use geom::cover::Cover;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn sym(&mut self) -> f64 {
        2.0 * self.next() - 1.0
    }
}

fn frame<T: Real>(o: [f64; 3], a: [f64; 3], u: [f64; 3]) -> Frame<T> {
    let v = |x: [f64; 3]| Vec3::new(T::from_f64(x[0]), T::from_f64(x[1]), T::from_f64(x[2]));
    Frame::new(
        Point3::new(T::from_f64(o[0]), T::from_f64(o[1]), T::from_f64(o[2])),
        v(a),
        v(u),
    )
}

/// The gap `outer.least − inner.greatest` about the outer's core.
fn gap<T: Real>(fo: &Frame<T>, fi: &Frame<T>, big: f64, r_out: f64, r_in: f64) -> T {
    let outer = Cover::torus(fo, T::from_f64(big), T::from_f64(r_out)).unwrap();
    let inner = Cover::torus(fi, T::from_f64(big), T::from_f64(r_in)).unwrap();
    let site = outer.about().into_iter().next().unwrap();
    outer.range_about(&site).0 - inner.range_about(&site).1
}

fn unit(x: [f64; 3]) -> [f64; 3] {
    let n = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
    [x[0] / n, x[1] / n, x[2] / n]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// **The `f64` depth gap of two nested tori is never over the true
/// gap, and the `Interval` one's lower end is never past it.** Coaxial
/// tori of tube radii `0.225·R` and `0.2125·R` in 5,000 frames at each
/// of four sizes, out to the session box's edge. A gap read over the
/// truth is the unsound side: it could part faces nearer than it says.
/// The `f64` reading charges no rounding (`[simple.conditioned]`), so
/// this is measured, not proved
/// (`work/embed/the-depth-gaps-f64-rounding-is-of-the-pads-order-near-the-least-tolerance.md`).
/// The frames are one fixed stream: the row holds the family that was
/// measured, and a search would vary them.
#[test]
fn a_nested_tori_depth_gap_at_f64_is_never_over_the_true_gap() {
    let mut rng = Lcg(7119);
    for &(scale, big) in &[(1.0, 2.0), (1e3, 2.0), (1e3, 400.0), (1e6, 1e6)] {
        let (mut worst_over, mut worst_under, mut widest) = (f64::MIN, f64::MAX, 0.0f64);
        for _ in 0..5000 {
            let o = [scale * rng.sym(), scale * rng.sym(), scale * rng.sym()];
            let a = unit([rng.sym(), rng.sym(), rng.sym()]);
            let u = unit(cross(a, unit([rng.sym(), rng.sym(), rng.sym()])));
            // The inner torus's frame: the same axis, its seam turned.
            let w = cross(a, u);
            let t = rng.sym() * 3.0;
            let u2 = unit([
                t.cos() * u[0] + t.sin() * w[0],
                t.cos() * u[1] + t.sin() * w[1],
                t.cos() * u[2] + t.sin() * w[2],
            ]);
            let (r_out, r_in) = (0.45 * big / 2.0, 0.425 * big / 2.0);
            let g = gap::<f64>(&frame(o, a, u), &frame(o, a, u2), big, r_out, r_in);
            let gi = gap::<Interval>(&frame(o, a, u), &frame(o, a, u2), big, r_out, r_in);
            let truth = r_out - r_in;
            worst_over = worst_over.max(g - truth);
            worst_under = worst_under.min(g - truth);
            widest = widest.max(gi.hi() - gi.lo());
            assert!(
                g <= truth,
                "scale {scale:e} R {big:e}: the f64 gap {g:e} is over the true gap {truth:e}"
            );
            assert!(
                gi.lo() <= truth + 1e-9 * big,
                "scale {scale:e} R {big:e}: the interval gap's lower end {:e} is past the true \
                 gap {truth:e}",
                gi.lo()
            );
        }
        println!(
            "scale {scale:e} R {big:e}: f64 gap - truth in [{worst_under:e}, {worst_over:e}], widest interval {widest:e}"
        );
    }
}
