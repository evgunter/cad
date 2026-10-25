//! Review probes for PR 5685 (D292): adversarial rows against
//! `ArcWindow` through both directional doors and the box doors.
//! Probe branch only.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom::curves::boxes::{
    ArcWindow, circle_arc_aabb, conic_arc_extent_along, conic_arc_extent_along_bracketed,
    ellipse_arc_aabb,
};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
}

/// The reference extent of `c + p cos t + q sin t` over [t0, t1]
/// (either orientation): ends plus every crest/trough angle in span,
/// plus 2000 samples. f64 evaluation.
fn reference(c: f64, p: f64, q: f64, t0: f64, t1: f64) -> (f64, f64) {
    let (a, b) = (t0.min(t1), t0.max(t1));
    let read = |t: f64| c + p * t.cos() + q * t.sin();
    let mut lo = read(t0).min(read(t1));
    let mut hi = read(t0).max(read(t1));
    let ts = q.atan2(p);
    for base in [ts, ts + PI] {
        let k0 = ((a - base) / TAU).ceil() as i64;
        let mut k = k0;
        while base + TAU * k as f64 <= b {
            let t = base + TAU * k as f64;
            lo = lo.min(read(t));
            hi = hi.max(read(t));
            k += 1;
            if k - k0 > 10 {
                break;
            }
        }
    }
    for i in 0..=2000 {
        let t = a + (b - a) * f64::from(i) / 2000.0;
        lo = lo.min(read(t));
        hi = hi.max(read(t));
    }
    (lo, hi)
}

/// ArcWindow at Interval, wide brackets on c, p, q, t0, t1: the door
/// must contain every sampled realization. Reports the worst
/// violation (scaled) and the count of undecided ties seen.
#[test]
fn probe_interval_window_contains_realizations_of_wide_brackets() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut worst: f64 = 0.0;
    let mut cases = 0usize;
    let mut undecided = 0usize;
    let widths = [0.0, 1e-12, 1e-9, 1e-6, 1e-3, 0.1, 0.5];
    // span shapes as (start offset from crest, length)
    for iter in 0..20_000 {
        let w = widths[iter % widths.len()];
        let amp_scale = [1.0, 1e-6, 1e3][iter % 3];
        let c = [0.0, 1.3, -115.0, 1e6][iter / 3 % 4];
        let (p, q) = if iter % 11 == 0 {
            // 5000:1 and worse
            (
                rng.range(-1.0, 1.0) * amp_scale,
                rng.range(-1.0, 1.0) * amp_scale * 1e-7,
            )
        } else {
            (
                rng.range(-1.0, 1.0) * amp_scale,
                rng.range(-1.0, 1.0) * amp_scale,
            )
        };
        let ts = q.atan2(p);
        let len = match iter % 9 {
            0 => PI,
            1 => TAU,
            2 => TAU + 1e-9,
            3 => 3.0 * TAU + 0.2,
            4 => TAU - 1e-7,
            5 => rng.range(0.0, 7.0),
            6 => 0.0,
            7 => -rng.range(0.0, 7.0),
            _ => -TAU,
        };
        let start = match iter / 9 % 6 {
            0 => ts,         // crest at start
            1 => ts - len,   // crest at end
            2 => ts + 1e-12, // crest just outside start (if len > 0)
            3 => ts - len - 1e-12,
            4 => ts + PI - len / 2.0, // trough centered
            _ => rng.range(-20.0, 20.0),
        };
        let (t0, t1) = (start, start + len);
        let wid = |x: f64, s: f64| Interval::from_bounds(x - w * s, x + w * s);
        let win = ArcWindow::new(wid(t0, 1.0), wid(t1, 1.0));
        let (ci, pi, qi) = (wid(c, 1.0), wid(p, amp_scale), wid(q, amp_scale));
        let co = win.crest_outside(pi, qi);
        if co.lo() <= 0.0 && co.hi() > 0.0 {
            undecided += 1;
        }
        let (lo, hi) = win.extent(ci, pi, qi);
        let (lo, hi) = (lo.lo(), hi.hi());
        // realizations: corners and random interior points
        for r in 0..12 {
            let pick = |x: f64, s: f64, rng: &mut Rng| match r {
                0 => x - w * s,
                1 => x + w * s,
                2 => x,
                _ => x + w * s * rng.range(-1.0, 1.0),
            };
            let (cr, pr, qr) = (
                pick(c, 1.0, &mut rng),
                pick(p, amp_scale, &mut rng),
                pick(q, amp_scale, &mut rng),
            );
            let (t0r, t1r) = (pick(t0, 1.0, &mut rng), pick(t1, 1.0, &mut rng));
            let (slo, shi) = reference(cr, pr, qr, t0r, t1r);
            let scale = cr.abs() + (pr.hypot(qr)) + 1e-300;
            let slack = 4.0 * f64::EPSILON * scale;
            let v = ((lo - slo).max(shi - hi)) / scale;
            cases += 1;
            if v * scale > slack {
                worst = worst.max(v);
                panic!(
                    "iter {iter}: w={w:e} c={c} p={p} q={q} t=[{t0},{t1}] realization \
                     ({cr},{pr},{qr},{t0r},{t1r}): door [{lo},{hi}] vs ref [{slo},{shi}]"
                );
            }
        }
    }
    eprintln!("probe interval: {cases} realizations, worst {worst:e}, undecided ties {undecided}");
    assert!(undecided > 100, "undecided ties exercised: {undecided}");
}

/// f64 door vs reference: exact to rounding (both sides), at the
/// adversarial span shapes.
#[test]
fn probe_f64_window_is_exact_to_rounding() {
    let mut rng = Rng(0xDEAD_BEEF_1234_5678);
    let mut worst: f64 = 0.0;
    let mut worst_at = String::new();
    let mut gross = std::collections::BTreeMap::new();
    for iter in 0..200_000 {
        let amp_scale = [1.0, 1e-6, 1e3][iter % 3];
        let c = [0.0, 1.3, -115.0, 1e6][iter / 3 % 4];
        let (p, q) = if iter % 11 == 0 {
            (
                rng.range(-1.0, 1.0) * amp_scale,
                rng.range(-1.0, 1.0) * amp_scale * 2e-4,
            )
        } else {
            (
                rng.range(-1.0, 1.0) * amp_scale,
                rng.range(-1.0, 1.0) * amp_scale,
            )
        };
        let ts = q.atan2(p);
        let len = match iter % 9 {
            0 => PI,
            1 => TAU,
            2 => TAU + 1e-9,
            3 => 3.0 * TAU + 0.2,
            4 => TAU - 1e-4,
            5 => rng.range(0.0, 7.0),
            6 => 0.0,
            7 => -rng.range(0.0, 7.0),
            _ => rng.range(1e-9, 1e-3),
        };
        let start = match iter / 9 % 7 {
            0 => ts,
            1 => ts - len,
            2 => ts + 1e-4,
            3 => ts - len - 1e-4,
            4 => ts + PI - len / 2.0,
            5 => ts + 1e-4 - TAU + 1e-4 / 2.0 - len + TAU, // crest in a near-full gap
            _ => rng.range(-20.0, 20.0),
        };
        let (t0, t1) = (start, start + len);
        let (lo, hi) = ArcWindow::new(t0, t1).extent(c, p, q);
        let (slo, shi) = reference(c, p, q, t0, t1);
        let a = p.hypot(q);
        let scale = c.abs() + a;
        // either side, in units of eps*scale
        let d = (lo - slo).abs().max((hi - shi).abs()) / (f64::EPSILON * scale);
        if d > 16.0 {
            let past = (t1 - t0).abs() > TAU;
            *gross
                .entry((past, iter % 9, iter / 9 % 7))
                .or_insert(0usize) += 1;
            if !past {
                eprintln!(
                    "NOT-PAST-A-TURN miss iter {iter}: c={c} p={p} q={q} t=[{t0},{t1}] door [{lo},{hi}] ref [{slo},{shi}] d={d}"
                );
            }
            continue;
        }
        if d > worst {
            worst = d;
            worst_at = format!(
                "iter {iter}: c={c} p={p} q={q} t=[{t0},{t1}] door [{lo},{hi}] ref [{slo},{shi}]"
            );
        }
    }
    eprintln!("probe f64: worst (non-gross) {worst} eps*scale at {worst_at}");
    eprintln!("probe f64: gross misses keyed (past a turn, len shape, start shape): {gross:?}");
    assert!(
        gross.keys().all(|k| k.0),
        "a gross miss on a span within a turn"
    );
    assert!(
        worst < 16.0,
        "f64 door off by {worst} eps*scale: {worst_at}"
    );
}

/// The box doors with a 5000:1 / 1e8:1 ellipse in a tilted plane, a
/// near-full turn with the crest in the gap, and a micron circle at
/// |c| = 1e6 — certified containment of a dense sample.
#[test]
fn probe_box_doors_contain_adversarial_arcs() {
    let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
    let u = axis.orthonormal_basis().0;
    let cases: Vec<(&str, Curve3<f64>, f64, f64)> = vec![
        (
            "flat 5000",
            Curve3::Ellipse {
                center: Point3::new(-1.0, 0.5, 2.0),
                axis,
                semi_u: 5.0,
                semi_v: 1e-3,
                u_ref: u,
            },
            0.3,
            5.9,
        ),
        (
            "flat 1e8",
            Curve3::Ellipse {
                center: Point3::new(-1.0, 0.5, 2.0),
                axis,
                semi_u: 5.0,
                semi_v: 5e-8,
                u_ref: u,
            },
            0.3,
            5.9,
        ),
        (
            "near full",
            Curve3::Circle {
                center: Point3::new(0.3, -0.2, 0.15),
                axis,
                radius: 1.0,
                u_ref: u,
            },
            0.1,
            0.1 + 0.999 * TAU,
        ),
        (
            "micron far",
            Curve3::Circle {
                center: Point3::new(1e6, -5e5, 2.5e5),
                axis,
                radius: 1e-6,
                u_ref: u,
            },
            0.7,
            2.9,
        ),
        (
            "descending past turn",
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, -3.0),
                axis,
                radius: 2.5,
                u_ref: u,
            },
            9.0,
            0.37 - TAU,
        ),
        (
            "exactly pi",
            Curve3::Circle {
                center: Point3::new(0.0, 1.0, -3.0),
                axis,
                radius: 2.5,
                u_ref: u,
            },
            0.37,
            0.37 + PI,
        ),
    ];
    for (what, c, t0, t1) in cases {
        let b = match c {
            Curve3::Circle { .. } => circle_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1)),
            _ => ellipse_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1)),
        }
        .unwrap();
        let mut wmax: f64 = 0.0;
        for i in 0..=200_000 {
            let t = t0 + (t1 - t0) * f64::from(i) / 200_000.0;
            let p = c.eval(t);
            let s = 4.0 * f64::EPSILON * (1.0 + p.x.abs().max(p.y.abs()).max(p.z.abs()));
            for (w, lo, hi) in [
                (p.x, b.min_x, b.max_x),
                (p.y, b.min_y, b.max_y),
                (p.z, b.min_z, b.max_z),
            ] {
                assert!(
                    w >= lo - s && w <= hi + s,
                    "{what}: t={t} {w} outside [{lo},{hi}]"
                );
                wmax = wmax.max((lo - w).max(w - hi));
            }
        }
        // the oblique directional doors too
        for n in [Vec3::new(0.3, -0.8, 0.5), Vec3::new(-0.9, 0.1, 0.4)] {
            let (blo, bhi) =
                conic_arc_extent_along_bracketed(&c, t0, t1, Point3::origin(), n).unwrap();
            let (flo, fhi) = conic_arc_extent_along(&c, t0, t1, Point3::origin(), n).unwrap();
            for i in 0..=200_000 {
                let t = t0 + (t1 - t0) * f64::from(i) / 200_000.0;
                let r = (c.eval(t) - Point3::origin()).dot(n);
                let s = 8.0 * f64::EPSILON * (1.0 + r.abs());
                assert!(
                    r >= blo - s && r <= bhi + s,
                    "{what} certified along {n:?}: {r} outside [{blo},{bhi}]"
                );
                assert!(
                    r >= flo - s && r <= fhi + s,
                    "{what} f64 along {n:?}: {r} outside [{flo},{fhi}]"
                );
            }
        }
        eprintln!("probe box {what}: {b:?} worst sample overshoot {wmax:e}");
    }
}

/// Undecided tie at Interval, made to straddle: a crest at the end of
/// a span whose end bracket straddles it, and h straddling pi (span
/// ~ tau with a wide bracket) with the crest in the gap. Prints the
/// door against the two candidate hulls.
#[test]
fn probe_interval_ties_and_h_straddling_pi() {
    let (p, q) = (0.6, 0.8); // crest at atan2(0.8, 0.6)
    let ts = q.atan2(p);
    for w in [1e-12, 1e-6, 1e-2] {
        // h straddles pi: span [ts + 0.01, ts + 0.01 + tau]; crest at ts+tau is inside
        // for realizations of length >= tau - 0.01... and the gap
        for (t0, t1) in [
            (ts + w / 2.0, ts + TAU - w / 2.0),
            (ts + TAU - w / 2.0, ts + w / 2.0),
        ] {
            let wid = |x: f64| Interval::from_bounds(x - w, x + w);
            let win = ArcWindow::new(wid(t0), wid(t1));
            let co = win.crest_outside(wid(p), wid(q));
            let (lo, hi) = win.extent(wid(1.0), wid(p), wid(q));
            eprintln!(
                "w={w:e} [{t0},{t1}]: crest_outside [{}, {}], extent [{}, {}]",
                co.lo(),
                co.hi(),
                lo.lo(),
                hi.hi()
            );
            // the realization with the crest in: max = 1 + A
            assert!(
                hi.hi() >= 1.0 + (p.hypot(q)) - 2.0 * w - 1e-15,
                "dropped the crest"
            );
            assert!(lo.lo() <= 1.0 - (p.hypot(q)) + 1e-15, "dropped the trough");
        }
    }
}

/// **Pinned counterexample** (found by the f64 probe above, iter
/// 53654): past a turn, a crest antipodal to the span's middle ties
/// `crest_outside` at f64 (`cos h` is clamped to −1), rounding reads it
/// as `+2.3e-13`, outside, and the door returns an end, short of the
/// crest by `A·(1 − cos(h − π))` ≈ 65 of A ≈ 1055.
#[test]
fn probe_f64_door_drops_the_crest_antipodal_to_the_middle_past_a_turn() {
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let n = Vec3::new(730.3003055298171, 761.676099383592, 0.0);
    let (t0, t1) = (0.4539818544898191, 7.442052950951721);
    let a = n.norm();
    let (lo, hi) = conic_arc_extent_along(&circle, t0, t1, Point3::origin(), n).unwrap();
    let (blo, bhi) =
        conic_arc_extent_along_bracketed(&circle, t0, t1, Point3::origin(), n).unwrap();
    eprintln!("f64 door [{lo}, {hi}], certified [{blo}, {bhi}], true [-{a}, {a}]");
    assert!(bhi >= a && blo <= -a, "certified door unsound");
    assert!(
        hi >= a * (1.0 - 1e-12),
        "f64 door dropped the crest past a turn: hi {hi} vs the crest {a}"
    );
}
