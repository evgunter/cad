//! **`Pcurve::chart_box` on a harmonic image, swept**: the span box
//! encloses the image at both scalars and is restriction-monotone,
//! over draws past `pcurve_cache`'s own nine-row table — far-from-zero
//! `t`, spans from 1e-9 to 20 rad in either direction, and channels
//! that are purely linear, purely trigonometric, or both.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom-brep/src/pcurve_cache.rs"];

use geom_brep::{ChartWindow, Pcurve};
use geom_core::{Bounds, Interval, Point2, Real, Vec2};
use test_utils::fuzz;

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

/// One image and span. Each channel independently carries a linear
/// part, a trigonometric part, or both.
fn draw(g: &mut fuzz::Rng) -> (Coeffs, f64, f64) {
    let (mut p0, mut pa, mut pb, mut pl) = ([0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2]);
    for i in 0..2 {
        p0[i] = g.range(-3.0, 3.0);
        let shape = g.below(3);
        if shape != 0 {
            pa[i] = g.range(-3.0, 3.0);
            pb[i] = g.range(-3.0, 3.0);
        }
        if shape != 1 {
            pl[i] = g.range(-3.0, 3.0);
        }
    }
    let scale = [1e-9, 1e-3, 0.5, 3.0, 7.0, 20.0][g.below(6)];
    let t0 = g.range(-1.0, 1.0) * [1.0, 50.0, 1e4][g.below(3)];
    let t1 = t0 + g.range(-1.0, 1.0) * scale;
    ((p0, pa, pb, pl), t0, t1)
}

/// The rounding scale of a draw's chart values at `f64`.
fn magnitude(c: &Coeffs, t0: f64, t1: f64) -> f64 {
    let reach = t0.abs().max(t1.abs());
    (0..2)
        .map(|i| 1.0 + c.0[i].abs() + c.1[i].abs() + c.2[i].abs() + c.3[i].abs() * reach)
        .fold(0.0, f64::max)
}

fn samples(t0: f64, t1: f64) -> impl Iterator<Item = f64> {
    // Clamped into the span: `t0 + (t1 − t0)·k` can round past its end.
    let (lo, hi) = (t0.min(t1), t0.max(t1));
    (0..=200).map(move |i| (t0 + (t1 - t0) * f64::from(i) / 200.0).clamp(lo, hi))
}

/// At `Interval` no sample's image enclosure lies certainly outside
/// the box: the true point is in both, so a box end's bracket past the
/// far end of the image's is a miss. (The near ends may cross by
/// rounding — the two are different evaluations of one real.)
#[test]
fn the_span_box_encloses_the_image_at_interval() {
    let mut g = fuzz::start("chart_box_span::interval");
    for _ in 0..fuzz::scaled(300) {
        let (c, t0, t1) = draw(&mut g);
        let p = pcurve::<Interval>(&c);
        let b = p.chart_box(Interval::from_f64(t0), Interval::from_f64(t1));
        for t in samples(t0, t1) {
            let at = p.eval(Interval::from_f64(t));
            for (lo, hi, x) in [(b.u_min, b.u_max, at.x), (b.v_min, b.v_max, at.y)] {
                assert!(
                    lo.lo() <= x.hi() && hi.hi() >= x.lo(),
                    "{c:?} [{t0}, {t1}] at {t}: box [{}, {}] misses the image [{}, {}] ({})",
                    lo.lo(),
                    hi.hi(),
                    x.lo(),
                    x.hi(),
                    fuzz::replay()
                );
            }
        }
    }
}

/// At `f64` the box misses the image only at rounding scale.
#[test]
fn the_span_box_misses_the_image_only_by_rounding_at_f64() {
    let mut g = fuzz::start("chart_box_span::f64");
    for _ in 0..fuzz::scaled(300) {
        let (c, t0, t1) = draw(&mut g);
        let p = pcurve::<f64>(&c);
        let b = p.chart_box(t0, t1);
        let slack = 1e-14 * magnitude(&c, t0, t1);
        for t in samples(t0, t1) {
            let at = p.eval(t);
            for (lo, hi, x) in [(b.u_min, b.u_max, at.x), (b.v_min, b.v_max, at.y)] {
                assert!(
                    lo - slack <= x && x <= hi + slack,
                    "{c:?} [{t0}, {t1}] at {t}: box [{lo}, {hi}] misses {x} ({})",
                    fuzz::replay()
                );
            }
        }
    }
}

fn escape(child: &ChartWindow<f64>, parent: &ChartWindow<f64>) -> f64 {
    (parent.u_min - child.u_min)
        .max(child.u_max - parent.u_max)
        .max(parent.v_min - child.v_min)
        .max(child.v_max - parent.v_max)
}

/// **Restriction monotonicity**: both halves of a split span box
/// inside the whole span's box, up to rounding — what a split edge's
/// halves, re-certified against a window holding the parent's box,
/// rely on (`topo::pcurves::split_cache`).
#[test]
fn a_sub_span_box_lies_inside_the_span_box() {
    let mut g = fuzz::start("chart_box_span::restriction");
    for _ in 0..fuzz::scaled(4000) {
        let (c, t0, t1) = draw(&mut g);
        let t = t0 + (t1 - t0) * g.unit();
        let p = pcurve::<f64>(&c);
        let parent = p.chart_box(t0, t1);
        let slack = 1e-14 * magnitude(&c, t0, t1);
        for (a, b) in [(t0, t), (t, t1)] {
            let esc = escape(&p.chart_box(a, b), &parent);
            assert!(
                esc <= slack,
                "{c:?}: [{a}, {b}]'s box leaves [{t0}, {t1}]'s by {esc} ({})",
                fuzz::replay()
            );
        }
    }
}
