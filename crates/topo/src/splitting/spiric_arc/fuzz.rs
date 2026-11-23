//! The counterexample-SEARCH half of the spiric crossing row: tori,
//! arcs, points and rays nobody chose, against the carrier's own
//! derivatives and a dense polyline. Folded in from PR 3924's review.

// Gated to the code it tests: the row, the bounds it reads, the
// carrier it evaluates, and the band and arithmetic it decides in.
test_utils::gated_to![
    "crates/topo/src/splitting/spiric_arc.rs",
    "crates/topo/src/splitting/containment.rs",
    "crates/geom-brep/src/curve_face/spiric_plane.rs",
    "crates/geom-brep/src/curve_face/pose.rs",
    "crates/geom-core/src/bisect.rs",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/linalg/",
];

use super::*;
use crate::splitting::pieced_arc::{ArcHit, ArcRows};
use geom_core::{Interval, Tol, Vec3};
use test_utils::fuzz::Rng;

const ROWS: ArcRows = crate::rows::FUZZ_SPIRIC_ARC;

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

/// Every regime the rate bounds cover: near-tangent cuts
/// (`|o| → R − r`), very thin and very fat rings, negative offsets, and
/// by the axis: `through_axis`, zero offset (a plane through the axis —
/// [`geom::Curve3::spiric`] refuses it, the bounds' algebra does not);
/// otherwise a small one, inside the two-oval convention. Either way the
/// draws are the same.
fn torus(rng: &mut Rng, through_axis: bool) -> Torus {
    let big = 10f64.powf(rng.range(-2.0, 2.0));
    let r = big
        * match rng.below(4) {
            0 => 10f64.powf(rng.range(-4.0, -1.0)),
            1 => 1.0 - 10f64.powf(rng.range(-4.0, -1.0)),
            _ => rng.range(0.05, 0.95),
        };
    let ring = big - r;
    let frac = match rng.below(5) {
        0 => {
            let small = 10f64.powf(rng.range(-6.0, -2.0));
            if through_axis { 0.0 } else { small }
        }
        1 => 1.0 - 10f64.powf(rng.range(-6.0, -2.0)),
        _ => 0.001 + rng.unit() * 0.998,
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
        frame: geom::Frame::new(Point3::new(f(c.x), f(c.y), f(c.z)), v3(axis), v3(u)),
        major_radius: f(t.big),
        minor_radius: f(t.r),
        offset: f(t.o),
    }
}

fn oval<T: Decide>(t: Torus, c: Point3<f64>, (axis, u): (Vec3<f64>, Vec3<f64>)) -> Oval<T> {
    let f = T::from_f64;
    let v3 = |v: Vec3<f64>| Vec3::new(f(v.x), f(v.y), f(v.z));
    Oval {
        frame: geom::Frame::new(Point3::new(f(c.x), f(c.y), f(c.z)), v3(axis), v3(u)),
        major: f(t.big),
        minor: f(t.r),
        offset: f(t.o),
    }
}

/// **`|P′| ≤ S` and `|P″| ≤ A` over every admitted regime, on every
/// window**: random tori — zero offset, near-tangent cuts
/// (`|o| → R − r`), very thin and very fat rings, negative offsets —
/// and on each a whole period and random pieces, sampled densely
/// against the piece's own bounds.
#[test]
fn the_rate_bounds_hold_over_every_admitted_regime() {
    let mut rng = test_utils::fuzz::start("spiric_bounds");
    let (mut worst_a, mut worst_s) = (0.0_f64, 0.0_f64);
    for _ in 0..test_utils::fuzz::scaled(200) {
        let t = torus(&mut rng, true);
        let k = SpiricArc::new(
            oval::<f64>(t, Point3::new(0.0, 0.0, 0.0), frame(&mut rng)),
            (0.0, 1.0),
        );
        let c = geom::Curve3::Spiric {
            frame: k.oval.frame,
            major_radius: t.big,
            minor_radius: t.r,
            offset: t.o,
        };
        let mut windows = vec![(0.0, core::f64::consts::TAU)];
        for _ in 0..8 {
            let va = rng.range(-7.0, 7.0);
            windows.push((va, va + 10f64.powf(rng.range(-4.0, 0.5))));
        }
        for (va, vb) in windows {
            let p = Piece {
                va,
                vb,
                pa: k.point(va),
                pb: k.point(vb),
                depth: 0,
            };
            let h = 0.5 * (vb - va).abs();
            let speed = k.bounds(&p).reach / h;
            let (_, accel) =
                geom::spiric_window_rate_bounds((t.big, t.r, t.o), geom::mid_param(va, vb), h);
            for i in 0..=400 {
                let v = va + (vb - va) * f64::from(i) / 400.0;
                let a = c.deriv2(v).norm() / accel;
                let s = c.deriv(v).norm() / speed;
                worst_a = worst_a.max(a);
                worst_s = worst_s.max(s);
                assert!(
                    a <= 1.0 + 1e-12 && s <= 1.0 + 1e-12,
                    "{t:?} on [{va}, {vb}] at {v}: |P″|/A = {a}, |P′|/S = {s}; {}",
                    test_utils::fuzz::replay()
                );
            }
        }
    }
    println!("worst |P″|/A = {worst_a}, worst |P′|/S = {worst_s}");
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
fn random_arcs<T: Decide>(lane: &str, cases: usize) {
    let band = band();
    let mut rng = test_utils::fuzz::start(&format!("spiric_arcs_{lane}"));
    let (mut asked, mut abandoned, mut refused_far, mut near_asked) =
        (0usize, 0usize, 0usize, 0usize);
    let mut wrong = Vec::new();
    let mut on_refused = 0usize;
    let mut unminted = 0usize;
    for case in 0..cases {
        let t = torus(&mut rng, false);
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
        // The ray's crossings read an edge's span, which ascends; the
        // boundary reading has no sense, and reads it either way.
        let reversed = rng.below(2) == 1;
        let span = (v0, v0 + w);
        let cs = Case { t, fr, c };
        let cf: geom::Curve3<f64> = carrier(cs.t, cs.c, cs.fr);
        let k = SpiricArc::new(
            oval::<T>(cs.t, cs.c, cs.fr),
            (T::from_f64(span.0), T::from_f64(span.1)),
        );
        let contact_arc = if reversed {
            SpiricArc::new(
                oval::<T>(cs.t, cs.c, cs.fr),
                (T::from_f64(span.1), T::from_f64(span.0)),
            )
        } else {
            k
        };
        // A cut the mint refuses at this band (its regime in band, or
        // read past the inner equator): a ray across it is refused typed
        // on the regime's margin, never read and never a fault.
        if geom::Curve3::<f64>::spiric(c, fr.0, fr.1, t.big, t.r, t.o, band).is_err() {
            unminted += 1;
            let got = k.crossings(
                crate::geometry::SurfaceKey::default(),
                false,
                normal_t::<T>(fr.1),
                (
                    Point3::new(T::from_f64(c.x), T::from_f64(c.y), T::from_f64(c.z)),
                    normal_t::<T>(fr.0),
                ),
                band,
            );
            if got.is_ok() {
                wrong.push(format!(
                    "{lane} case {case} {t:?}: a cut the mint refuses read {got:?}"
                ));
            }
            continue;
        }
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
            match contact_arc.contact(lift(q), ROWS, band) {
                Ok(ArcHit::Off) => {}
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
            match k.crossings(
                crate::geometry::SurfaceKey::default(),
                false,
                normal_t::<T>(normal),
                (lift(q), d),
                band,
            ) {
                Ok(None) | Err(_) => abandoned += 1,
                Ok(Some(cnt)) => {
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
            let got = contact_arc.contact(lift(q), ROWS, band);
            if dd <= band.escalate() && got == Ok(ArcHit::Off) {
                wrong.push(format!(
                    "{lane} case {case} {:?}: δ={delta} in band read Off",
                    cs.t
                ));
            }
            if dd < 0.5 * band.zero() && matches!(got, Ok(ArcHit::Off)) {
                wrong.push(format!(
                    "{lane} case {case} {:?}: δ={delta} below zero read {got:?}",
                    cs.t
                ));
            }
            if dd < 0.5 * band.zero() && got.is_err() {
                on_refused += 1;
            }
            if dd > 2.0 * band.escalate() && got != Ok(ArcHit::Off) {
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
        "{lane}: {asked} far asked, {abandoned} rays abandoned, {near_asked} near asked, {refused_far} refused past band, {on_refused} on-arc refused, {unminted} cuts the mint refuses"
    );
    // A ray is set aside near a graze or an arc's end; a lane that sets
    // most aside reads nothing.
    assert!(
        abandoned * 5 <= asked,
        "{lane}: {abandoned} of {asked} rays set aside"
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
fn random_arcs_at_f64() {
    random_arcs::<f64>("f64", test_utils::fuzz::scaled(40));
}

#[test]
fn random_arcs_at_interval() {
    random_arcs::<Interval>("Interval", test_utils::fuzz::scaled(4));
}
