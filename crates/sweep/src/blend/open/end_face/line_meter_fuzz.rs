//! The straight-edge cap meter's candidate set, swept: on random
//! enclosures and adversarial segments the meter reads the segment's
//! least `G` to rounding at `f64`, and at `Interval` its enclosure's
//! low end lies at or below that least and within rounding of the
//! `f64` reading. Against
//! the sliver itself only the segment's ends carry soundness, so no
//! assembly row can see a candidate go missing; this sweep is what
//! does.

test_utils::gated_to![
    "crates/sweep/src/blend/surgery.rs",
    "crates/geom-core/src/interval/"
];

use super::tests::Region;
use geom_core::{Bounds, Interval};
use test_utils::fuzz;

/// The least of `G` over `a → b`: the least of a dense run of samples,
/// refined by ternary search about it. Never below the true least.
fn sampled_least(region: &Region, a: (f64, f64), b: (f64, f64)) -> f64 {
    let at = |t: f64| region.g((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t));
    const N: u32 = 2000;
    let (best, n) = (0..=N)
        .map(|n| (at(f64::from(n) / f64::from(N)), n))
        .fold((f64::INFINITY, 0), |l, r| if r.0 < l.0 { r } else { l });
    let (mut lo, mut hi) = (
        f64::from(n.saturating_sub(1)) / f64::from(N),
        f64::from((n + 1).min(N)) / f64::from(N),
    );
    for _ in 0..100 {
        let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
        if at(m1) < at(m2) {
            hi = m2;
        } else {
            lo = m1;
        }
    }
    best.min(at(0.5 * (lo + hi)))
}

#[test]
fn the_line_meter_reads_the_least_of_g_on_random_segments() {
    let mut rng = fuzz::start("line_meter");
    for case in 0..fuzz::scaled(400) {
        let reach = rng.range(0.5, 1.5);
        let minor = reach * rng.range(0.3, 0.99);
        let mut floors: Vec<((f64, f64), f64)> = (0..rng.below(6))
            .map(|_| {
                let t = if rng.unit() < 0.3 {
                    std::f64::consts::FRAC_PI_2 * rng.below(4) as f64
                } else {
                    rng.range(0.0, std::f64::consts::TAU)
                };
                ((t.cos(), t.sin()), rng.range(-0.6, 0.4) * reach)
            })
            .collect();
        if floors.len() >= 2 && rng.unit() < 0.3 {
            // A repeated direction, or its opposite.
            let ((x, y), f) = floors[0];
            floors[1] = if rng.unit() < 0.5 {
                ((x, y), f)
            } else {
                ((-x, -y), -f - 0.1 * rng.unit())
            };
        }
        let region = Region {
            section: (rng.unit() < 0.7).then_some((0.0, minor, minor)),
            reach,
            floors,
        };
        let scale = [1e-9, 1e-4, 0.05, 1.0, 3.0, 1e4][rng.below(6)];
        let t = rng.range(0.0, std::f64::consts::TAU);
        let (mut a, mut u) = (
            (rng.range(-1.5, 1.5), rng.range(-1.5, 1.5)),
            (t.cos(), t.sin()),
        );
        let tangent = |r: f64, b: f64| ((r * b.cos(), r * b.sin()), (-b.sin(), b.cos()));
        match rng.below(8) {
            0 if !region.floors.is_empty() => u = region.floors[0].0,
            1 if !region.floors.is_empty() => {
                let ((x, y), f) = region.floors[0];
                let o = rng.range(-1.0, 1.0);
                (a, u) = ((x * f - y * o, y * f + x * o), (-y, x));
            }
            2 => a = (-u.0 * scale * rng.unit(), -u.1 * scale * rng.unit()),
            3 => (a, u) = tangent(reach, rng.range(0.0, std::f64::consts::TAU)),
            4 => (a, u) = tangent(minor, rng.range(0.0, std::f64::consts::TAU)),
            _ => {}
        }
        let back = scale * rng.unit();
        a = (a.0 - u.0 * back, a.1 - u.1 * back);
        let b = (a.0 + u.0 * scale, a.1 + u.1 * scale);
        let least = sampled_least(&region, a, b);
        // `G` is 1-Lipschitz, so the samples' least is within half a
        // spacing of the true one; rounding grows with the distance
        // from the centre.
        let slack = scale / 4000.0 + 1e-14 * (1.0 + a.0.hypot(a.1) + scale);
        let m: f64 = region.meter(a, b);
        let iv: Interval = region.meter(a, b);
        let what = || {
            format!(
                "case {case}: {a:?} → {b:?}, reach {reach}, section {:?}, floors {:?}; {}",
                region.section,
                region.floors,
                fuzz::replay()
            )
        };
        assert!(
            m <= least + slack && m >= least - slack,
            "f64 reads {m}, the least G is {least}: {}",
            what()
        );
        // The enclosure carries the gain: its low end is the `f64`
        // reading's to rounding, not the whole segment's.
        assert!(
            iv.lo() <= least + slack && iv.lo() >= m - 1e-12 * (1.0 + a.0.hypot(a.1) + scale),
            "Interval reads {iv:?}, f64 {m}, the least G is {least}: {}",
            what()
        );
    }
}
