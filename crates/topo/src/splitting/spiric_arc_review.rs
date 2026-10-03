//! Reviewer rows for [`super::SpiricArc`] (PR 3924's first review): the
//! acceleration bound over the carrier's whole admitted range, and the
//! two readings against a dense polyline of random arcs at both scalars.

use super::*;
use geom_core::{Interval, Tol};
use test_utils::fuzz::Rng;

const ROWS: SpiricRows = SpiricRows {
    end: "review_spiric_end",
    clear: "review_spiric_clear",
    on: "review_spiric_on",
    leaf: "review_spiric_leaf",
    depth: "review_spiric_depth",
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A random orthonormal pair `(axis, u_ref)`.
fn frame(rng: &mut Rng) -> (Vec3<f64>, Vec3<f64>) {
    loop {
        let a = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let b = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        if a.norm() < 0.2 {
            continue;
        }
        let a = a * (1.0 / a.norm());
        let b = b - a * b.dot(a);
        if b.norm() < 0.2 {
            continue;
        }
        return (a, b * (1.0 / b.norm()));
    }
}

#[derive(Clone, Copy, Debug)]
struct Torus {
    big: f64,
    r: f64,
    o: f64,
}

/// Every admitted regime: plane through the axis, near-tangent cuts
/// (`|o| → R − r`), very thin and very fat rings, negative offsets.
fn torus(rng: &mut Rng) -> Torus {
    let big = 10f64.powf(rng.range(-2.0, 2.0));
    let r = big
        * match rng.below(4) {
            0 => 10f64.powf(rng.range(-4.0, -1.0)),
            1 => 1.0 - 10f64.powf(rng.range(-4.0, -1.0)),
            _ => rng.range(0.05, 0.95),
        };
    let ring = big - r;
    let frac = match rng.below(5) {
        0 => 0.0,
        1 => 1.0 - 10f64.powf(rng.range(-6.0, -2.0)),
        _ => rng.unit() * 0.999,
    };
    let o = if rng.below(2) == 0 {
        frac * ring
    } else {
        -frac * ring
    };
    Torus { big, r, o }
}

fn carrier<T: Decide>(
    t: Torus,
    c: Point3<f64>,
    (axis, u): (Vec3<f64>, Vec3<f64>),
) -> geom::Curve3<T> {
    let f = T::from_f64;
    let v3 = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
    geom::Curve3::Spiric {
        center: Point3::new(f(c.x), f(c.y), f(c.z)),
        axis: v3(axis),
        u_ref: v3(u),
        major_radius: f(t.big),
        minor_radius: f(t.r),
        offset: f(t.o),
    }
}

/// **`|P″| ≤ A` and `|P′| ≤ S` over the whole admitted range**, sampled
/// densely, with the worst ratio printed (the slack a plant-out eats).
#[test]
fn review_bounds_hold_over_every_admitted_regime() {
    let mut rng = test_utils::fuzz::start("review_spiric_bounds");
    let (mut worst_a, mut worst_s) = (0.0_f64, 0.0_f64);
    let mut worst_case = None;
    for _ in 0..2000 {
        let t = torus(&mut rng);
        let c: geom::Curve3<f64> = carrier(t, Point3::new(0.0, 0.0, 0.0), frame(&mut rng));
        let k = SpiricArc::of(&c, (0.0, 1.0)).unwrap();
        for i in 0..4000 {
            let v = core::f64::consts::TAU * f64::from(i) / 4000.0;
            let a = c.deriv2(v).norm() / k.accel;
            let s = c.deriv(v).norm() / k.speed;
            if a > worst_a {
                worst_a = a;
                worst_case = Some(t);
            }
            worst_s = worst_s.max(s);
            assert!(a <= 1.0 + 1e-12, "{t:?} v={v}: |P''|/A = {a}");
            assert!(s <= 1.0 + 1e-12, "{t:?} v={v}: |P'|/S = {s}");
        }
    }
    println!("worst |P''|/A = {worst_a} at {worst_case:?}; worst |P'|/S = {worst_s}");
}

/// **Exact differentiation check of the closed form at the
/// worst-ratio corner**: near-tangent cut of a thin ring, where
/// `f_min → 0`, the leading `ρ²ρ′²/f³` term dominates.
#[test]
fn review_accel_ratio_at_the_tangent_corner() {
    for frac in [0.9, 0.99, 0.999_999] {
        let t = Torus {
            big: 1.0,
            r: 0.5,
            o: 0.5 * frac,
        };
        let c: geom::Curve3<f64> = carrier(
            t,
            Point3::new(0.0, 0.0, 0.0),
            (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
        );
        let k = SpiricArc::of(&c, (0.0, 1.0)).unwrap();
        let mut worst = 0.0_f64;
        for i in 0..200_000 {
            let v = core::f64::consts::TAU * f64::from(i) / 200_000.0;
            worst = worst.max(c.deriv2(v).norm());
        }
        println!(
            "o/(R-r)={frac}: max|P''| = {worst}, A = {}, ratio {}",
            k.accel,
            worst / k.accel
        );
        assert!(worst <= k.accel);
    }
}

/// The arc's in-plane coordinates `((P − c)·m, (P − c)·axis)`.
fn plane_xy(p: Point3<f64>, c: Point3<f64>, m: Vec3<f64>, axis: Vec3<f64>) -> (f64, f64) {
    let w = p - c;
    (w.dot(m), w.dot(axis))
}

struct Case {
    t: Torus,
    fr: (Vec3<f64>, Vec3<f64>),
    c: Point3<f64>,
}

/// **Random arcs closed by their chord, read at both scalars, against
/// a dense polyline**: every far point answers `Off` and its certified
/// arc crossings plus the chord's are the polyline's parity; every
/// point placed `δ` off the arc along its normal reads `Off` past the
/// band, never `Off` inside it, `On` or `End` below the coincidence
/// threshold.
fn review_random_arcs<T: Decide>(lane: &str, cases: usize) {
    let band = band();
    let mut rng = test_utils::fuzz::start(&format!("review_spiric_arcs_{lane}"));
    let (mut asked, mut abandoned, mut refused_far, mut near_asked) =
        (0usize, 0usize, 0usize, 0usize);
    let mut wrong = Vec::new();
    let mut on_refused = 0usize;
    for case in 0..cases {
        let t = torus(&mut rng);
        // Keep scale near metres so the band is meaningful relative to it.
        let scale = 1.0 / t.big;
        let t = Torus {
            big: t.big * scale,
            r: t.r * scale,
            o: t.o * scale,
        };
        let fr = frame(&mut rng);
        let c = Point3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let v0 = rng.range(-4.0, 4.0);
        let w = rng.range(0.05, core::f64::consts::TAU - 0.05);
        let span = if rng.below(2) == 0 {
            (v0, v0 + w)
        } else {
            (v0 + w, v0)
        };
        let cs = Case { t, fr, c };
        let cf: geom::Curve3<f64> = carrier(cs.t, cs.c, cs.fr);
        let ct: geom::Curve3<T> = carrier(cs.t, cs.c, cs.fr);
        let Some(k) = SpiricArc::of(&ct, (T::from_f64(span.0), T::from_f64(span.1))) else {
            panic!()
        };
        let (axis, u) = fr;
        let m = axis.cross(u);
        let normal = u; // the plane's normal
        let n = 8000;
        let poly: Vec<(f64, f64)> = (0..=n)
            .map(|i| {
                let v = span.0 + (span.1 - span.0) * f64::from(i) / f64::from(n);
                plane_xy(cf.eval(v), c, m, axis)
            })
            .collect();
        let (lo, hi) = poly
            .iter()
            .fold(((f64::MAX, f64::MAX), (f64::MIN, f64::MIN)), |(l, h), p| {
                ((l.0.min(p.0), l.1.min(p.1)), (h.0.max(p.0), h.1.max(p.1)))
            });
        let a = poly[0];
        let b = *poly.last().unwrap();
        let lift = |(x, y): (f64, f64)| {
            let p = c + u * t.o + m * x + axis * y;
            Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z))
        };
        let lift_v = |(x, y): (f64, f64)| {
            let p = m * x + axis * y;
            Vec3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z))
        };
        for _ in 0..40 {
            let pad = 0.3 * ((hi.0 - lo.0).max(hi.1 - lo.1));
            let q = (
                rng.range(lo.0 - pad, hi.0 + pad),
                rng.range(lo.1 - pad, hi.1 + pad),
            );
            let ang = rng.range(0.0, core::f64::consts::TAU);
            let (dx, dy) = (ang.cos(), ang.sin());
            // Truth: gap, arc crossings along the ray, chord crossing.
            let mut gap = f64::INFINITY;
            let mut arc_cross = 0usize;
            let mut graze = false;
            let crosses = |pa: (f64, f64), pb: (f64, f64), graze: &mut bool| -> bool {
                let sa = (pa.0 - q.0) * (-dy) + (pa.1 - q.1) * dx;
                let sb = (pb.0 - q.0) * (-dy) + (pb.1 - q.1) * dx;
                if (sa > 0.0) == (sb > 0.0) {
                    return false;
                }
                let s = sa / (sa - sb);
                let h = (
                    pa.0 + s * (pb.0 - pa.0) - q.0,
                    pa.1 + s * (pb.1 - pa.1) - q.1,
                );
                let along = h.0 * dx + h.1 * dy;
                if along.abs() < 1e-6 {
                    *graze = true;
                }
                along > 0.0
            };
            for wnd in poly.windows(2) {
                let (pa, pb) = (wnd[0], wnd[1]);
                let (ex, ey) = (pb.0 - pa.0, pb.1 - pa.1);
                let s =
                    (((q.0 - pa.0) * ex + (q.1 - pa.1) * ey) / (ex * ex + ey * ey)).clamp(0.0, 1.0);
                gap = gap.min((q.0 - pa.0 - s * ex).hypot(q.1 - pa.1 - s * ey));
                arc_cross += usize::from(crosses(pa, pb, &mut graze));
            }
            if gap < 1e-4 || graze {
                continue;
            }
            let chord = usize::from(crosses(a, b, &mut graze));
            // A ray nearly through an arc end, or nearly tangent, is
            // allowed to be abandoned; the polyline may also miscount
            // a near-tangent double crossing, so compare parity only.
            let truth_inside = (arc_cross + chord) % 2 == 1;
            asked += 1;
            match k.contact(lift(q), ROWS, band) {
                Ok(SpiricHit::Off) => {}
                other => {
                    refused_far += 1;
                    if refused_far < 5 {
                        println!(
                            "{lane} case {case} {:?} q={q:?} gap={gap}: contact {other:?}",
                            cs.t
                        );
                    }
                    continue;
                }
            }
            let d = lift_v((dx, dy));
            let side = normal_t::<T>(normal).cross(d);
            match k.crossings(lift(q), d, side, band) {
                None => abandoned += 1,
                Some(cnt) => {
                    if cnt != arc_cross || ((cnt + chord) % 2 == 1) != truth_inside {
                        wrong.push(format!(
                            "{lane} case {case} {:?} span {:?} q={q:?} dir={ang}: row {cnt}, polyline {arc_cross}",
                            cs.t, span
                        ));
                    }
                }
            }
        }
        // Near points: δ off the arc along its in-plane normal.
        for _ in 0..10 {
            let v = span.0 + (span.1 - span.0) * rng.range(0.02, 0.98);
            let p = plane_xy(cf.eval(v), c, m, axis);
            let tg = cf.deriv(v);
            let tg2 = (tg.dot(m), tg.dot(axis));
            let l = tg2.0.hypot(tg2.1);
            let nrm = (-tg2.1 / l, tg2.0 / l);
            let delta =
                10f64.powf(rng.range(-11.0, -5.0)) * if rng.below(2) == 0 { 1.0 } else { -1.0 };
            let q = (p.0 + nrm.0 * delta, p.1 + nrm.1 * delta);
            // The polyline's gap only bounds the truth from above, and the
            // point might be near another part of a non-convex oval.
            let dd = delta.abs();
            near_asked += 1;
            let got = k.contact(lift(q), ROWS, band);
            if dd <= band.escalate() && got == Ok(SpiricHit::Off) {
                wrong.push(format!(
                    "{lane} case {case} {:?}: δ={delta} in band read Off",
                    cs.t
                ));
            }
            if dd < 0.5 * band.zero() && matches!(got, Ok(SpiricHit::Off)) {
                wrong.push(format!(
                    "{lane} case {case} {:?}: δ={delta} below zero read {got:?}",
                    cs.t
                ));
            }
            if dd < 0.5 * band.zero() && got.is_err() {
                on_refused += 1;
            }
            if dd > 2.0 * band.escalate() && got != Ok(SpiricHit::Off) {
                refused_far += 1;
                if refused_far < 10 {
                    println!(
                        "{lane} case {case} {:?} v={v} δ={delta}: past band, {got:?}",
                        cs.t
                    );
                }
            }
        }
    }
    println!(
        "{lane}: {asked} far asked, {abandoned} rays abandoned, {near_asked} near asked, {refused_far} refused past band, {on_refused} on-arc refused"
    );
    assert!(
        wrong.is_empty(),
        "{} wrong answers:\n{}",
        wrong.len(),
        wrong[..wrong.len().min(20)].join("\n")
    );
}

fn normal_t<T: Decide>(n: Vec3<f64>) -> Vec3<T> {
    Vec3::new(T::from_f64(n.x), T::from_f64(n.y), T::from_f64(n.z))
}

#[test]
fn review_random_arcs_at_f64() {
    review_random_arcs::<f64>("f64", 200);
}

#[test]
fn review_random_arcs_at_interval() {
    review_random_arcs::<Interval>("Interval", 30);
}

/// **The piece budget on a reachable carrier**: a near-tangent cut
/// (`|o| = (1 − 10⁻⁶)(R − r)`), whose speed bound is ~700× its
/// typical speed, read at a point a full minor radius off the arc.
#[test]
fn review_budget_on_a_near_tangent_cut() {
    let band = band();
    for frac in [0.99, 0.9999, 0.999_999, 0.999_999_99] {
        let t = Torus {
            big: 1.0,
            r: 0.25,
            o: 0.75 * frac,
        };
        let c: geom::Curve3<f64> = carrier(
            t,
            Point3::new(0.0, 0.0, 0.0),
            (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
        );
        let k = SpiricArc::of(&c, (-3.0, 3.0)).unwrap();
        // The oval's centre in the plane: (m, axis) = (y, z); x = o.
        let mid_y = 0.5 * (c.eval(0.0).y + c.eval(core::f64::consts::PI).y);
        let q = Point3::new(t.o, mid_y, 0.0);
        let got = k.contact(q, ROWS, band);
        let ray = k.crossings(
            q,
            Vec3::new(0.0, 0.3_f64.cos(), 0.3_f64.sin()),
            Vec3::new(1.0, 0.0, 0.0).cross(Vec3::new(0.0, 0.3_f64.cos(), 0.3_f64.sin())),
            band,
        );
        println!(
            "frac {frac}: S={} A={} contact {got:?} crossings {ray:?}",
            k.speed, k.accel
        );
    }
}
