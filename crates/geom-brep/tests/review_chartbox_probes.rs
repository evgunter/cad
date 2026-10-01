//! Review probes for PR #3610 (`Pcurve::harmonic_span_box`): a seeded
//! fuzz of the span box's enclosure at both scalars, past the PR's nine
//! rows (far-from-zero `t`, sub-ulp-ish spans, mixed trig + linear
//! channels), and the restriction property a parameter split leans on.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{ChartWindow, Pcurve};
use geom_core::{Bounds, Interval, Point2, Real, Vec2};

/// A tiny deterministic generator (no dependency).
struct Lcg(u64);
impl Lcg {
    fn unit(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        x
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
}

type Coeffs = ([f64; 2], [f64; 2], [f64; 2], [f64; 2]);

fn pcurve<T: Real>(c: &Coeffs) -> Pcurve<T> {
    let f = T::from_f64;
    let v = |x: [f64; 2]| Vec2::new(f(x[0]), f(x[1]));
    Pcurve::Harmonic {
        p0: Point2::new(f(c.0[0]), f(c.0[1])),
        pa: v(c.1),
        pb: v(c.2),
        pl: v(c.3),
    }
}

fn draw(g: &mut Lcg, mixed: bool) -> (Coeffs, f64, f64) {
    let mut ch = || [g.range(-3.0, 3.0), g.range(-3.0, 3.0)];
    let p0 = ch();
    let pa = ch();
    let pb = ch();
    let pl = if mixed { ch() } else { [0.0, 0.0] };
    let scale = [1e-9, 1e-3, 0.5, 3.0, 7.0, 20.0][(g.unit() * 6.0) as usize % 6];
    let t0 = g.range(-1.0, 1.0) * [1.0, 50.0, 1e4][(g.unit() * 3.0) as usize % 3];
    let t1 = t0 + g.range(-1.0, 1.0) * scale;
    ((p0, pa, pb, pl), t0, t1)
}

/// The box encloses the image at `Interval` on every draw: each box
/// end's bracket lies outside the interval evaluation at 501 samples.
#[test]
fn review_span_box_encloses_at_interval_on_a_fuzz() {
    let mut g = Lcg(0x3610);
    for _ in 0..600 {
        let mixed = g.unit() < 0.5;
        let (c, t0, t1) = draw(&mut g, mixed);
        let p = pcurve::<Interval>(&c);
        let b = p.chart_box(Interval::from_f64(t0), Interval::from_f64(t1));
        for i in 0..=500 {
            let t = t0 + (t1 - t0) * f64::from(i) / 500.0;
            let at = p.eval(Interval::from_f64(t));
            for (lo, hi, x) in [(b.u_min, b.u_max, at.x), (b.v_min, b.v_max, at.y)] {
                assert!(
                    lo.lo() <= x.lo() && hi.hi() >= x.hi(),
                    "{c:?} [{t0}, {t1}] at {t}: box [{}, {}] vs image [{}, {}]",
                    lo.lo(),
                    hi.hi(),
                    x.lo(),
                    x.hi()
                );
            }
        }
    }
}

/// At `f64` the box misses the f64-evaluated image by at most rounding
/// scale; report the worst relative miss over the fuzz.
#[test]
fn review_span_box_f64_misses_only_at_rounding_scale() {
    let mut g = Lcg(0x3611);
    let mut worst: f64 = 0.0;
    for _ in 0..600 {
        let mixed = g.unit() < 0.5;
        let (c, t0, t1) = draw(&mut g, mixed);
        let p = pcurve::<f64>(&c);
        let b = p.chart_box(t0, t1);
        let mag = 1.0 + c.0[0].abs().max(c.0[1].abs()) + 6.0 + 6.0 * t0.abs().max(t1.abs());
        for i in 0..=500 {
            let t = t0 + (t1 - t0) * f64::from(i) / 500.0;
            let at = p.eval(t);
            for (lo, hi, x) in [(b.u_min, b.u_max, at.x), (b.v_min, b.v_max, at.y)] {
                let miss = (lo - x).max(x - hi).max(0.0) / mag;
                worst = worst.max(miss);
            }
        }
    }
    eprintln!("worst f64 relative miss: {worst:e}");
    assert!(worst < 1e-14, "worst f64 relative miss {worst:e}");
}

fn inside(child: &ChartWindow<f64>, parent: &ChartWindow<f64>) -> f64 {
    (parent.u_min - child.u_min)
        .max(child.u_max - parent.u_max)
        .max(parent.v_min - child.v_min)
        .max(child.v_max - parent.v_max)
}

/// **Restriction monotonicity**: a child span's box inside the parent
/// span's box — what `topo::pcurves::split_cache` leans on when it
/// re-certifies both children (check 5) against the hull of the face's
/// STORED rows, which still holds the parent's box. The old ball about
/// `p0` with reach `max|t|` was monotone by construction. Reports the
/// worst escape per family; asserts on the pure-trig family only.
#[test]
fn review_child_span_boxes_inside_parent_span_box() {
    for mixed in [false, true] {
        let mut g = Lcg(0x3612 + u64::from(mixed));
        let (mut worst, mut count, mut example) = (0.0_f64, 0, None);
        for _ in 0..20000 {
            let (c, t0, _) = draw(&mut g, mixed);
            let t1 = t0 + g.range(0.05, 4.0);
            let t = t0 + (t1 - t0) * g.unit();
            let p = pcurve::<f64>(&c);
            let parent = p.chart_box(t0, t1);
            for child in [p.chart_box(t0, t), p.chart_box(t, t1)] {
                let esc = inside(&child, &parent);
                if esc > 1e-12 {
                    count += 1;
                }
                if esc > worst {
                    worst = esc;
                    example = Some((c, t0, t1, t));
                }
            }
        }
        eprintln!(
            "mixed={mixed}: {count} child boxes escape the parent; worst {worst} at {example:?}"
        );
        if !mixed {
            assert_eq!(count, 0, "pure trig: worst {worst} at {example:?}");
        }
    }
}
