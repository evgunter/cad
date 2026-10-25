//! `ArcWindow` against adversarial spans: randomized sweeps of the
//! window at both scalars (wide brackets at [`Interval`], exactness at
//! `f64`, spans past a full turn through the carrier doors), and static
//! rows for the box doors on hard carriers, an undecided tie at
//! `h ≈ π`, and the pinned past-a-turn counterexample.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom/src/curves/",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
    "crates/bvh/src/aabb.rs",
];

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom::curves::boxes::{
    ArcWindow, circle_arc_aabb, conic_arc_extent_along, conic_arc_extent_along_bracketed,
    ellipse_arc_aabb,
};
use geom_core::{Bounds, Interval, Point3, Vec3};
use test_utils::fuzz;

/// The extent of `c + p·cos t + q·sin t` over `[t0, t1]` (either
/// orientation): the two ends, every crest and trough angle in the
/// span, and 64 samples as a check on that enumeration.
fn reference(c: f64, p: f64, q: f64, t0: f64, t1: f64) -> (f64, f64) {
    let (a, b) = (t0.min(t1), t0.max(t1));
    let read = |t: f64| c + p * t.cos() + q * t.sin();
    let mut lo = read(t0).min(read(t1));
    let mut hi = read(t0).max(read(t1));
    let ts = q.atan2(p);
    for base in [ts, ts + PI] {
        let mut t = base + TAU * ((a - base) / TAU).ceil();
        while t <= b {
            lo = lo.min(read(t));
            hi = hi.max(read(t));
            t += TAU;
        }
    }
    for i in 0..=64 {
        let t = a + (b - a) * f64::from(i) / 64.0;
        lo = lo.min(read(t));
        hi = hi.max(read(t));
    }
    (lo, hi)
}

/// A harmonic `(c, p, q)` with the amplitudes and offsets the sweeps
/// draw: unit, micro and kilo amplitudes, a flat `q ≪ p`, and offsets
/// from zero to `10⁶`.
fn harmonic(rng: &mut fuzz::Rng) -> (f64, f64, f64, f64) {
    let scale = [1.0, 1e-6, 1e3][rng.below(3)];
    let c = [0.0, 1.3, -115.0, 1e6][rng.below(4)];
    let flat = if rng.below(11) == 0 { 1e-7 } else { 1.0 };
    let p = rng.range(-1.0, 1.0) * scale;
    let q = rng.range(-1.0, 1.0) * scale * flat;
    (c, p, q, scale)
}

/// A span `[t0, t1]` placed against the crest angle `ts`: lengths of a
/// half, a full and several turns, just under and just over a turn,
/// empty, random either way; starts at the crest, ending at it, just
/// beside it, centring the trough, or anywhere.
fn span(rng: &mut fuzz::Rng, ts: f64) -> (f64, f64) {
    let len = match rng.below(9) {
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
    let start = match rng.below(6) {
        0 => ts,
        1 => ts - len,
        2 => ts + 1e-4,
        3 => ts - len - 1e-4,
        4 => ts + PI - len / 2.0,
        _ => rng.range(-20.0, 20.0),
    };
    (start, start + len)
}

/// **Sound at [`Interval`] over wide brackets.** Every scalar of the
/// harmonic and the span is widened to a bracket of width `w`; the
/// window's extent must contain the reference extent of every corner,
/// the centre and random members, up to the reference's own `f64`
/// rounding.
#[test]
fn the_interval_window_contains_every_member_of_a_wide_bracket() {
    let mut rng = fuzz::start("arc_window::interval_brackets");
    let widths = [0.0, 1e-12, 1e-9, 1e-6, 1e-3, 0.1, 0.5];
    for _ in 0..fuzz::scaled(2_000) {
        let w = widths[rng.below(widths.len())];
        let (c, p, q, scale) = harmonic(&mut rng);
        let (t0, t1) = span(&mut rng, q.atan2(p));
        let wid = |x: f64, s: f64| Interval::from_bounds(x - w * s, x + w * s);
        let window = ArcWindow::new(wid(t0, 1.0), wid(t1, 1.0));
        let (lo, hi) = window.extent(wid(c, 1.0), wid(p, scale), wid(q, scale));
        let (lo, hi) = (lo.lo(), hi.hi());
        for r in 0..12 {
            let mut pick = |x: f64, s: f64| match r {
                0 => x - w * s,
                1 => x + w * s,
                2 => x,
                _ => x + w * s * rng.range(-1.0, 1.0),
            };
            let (cr, pr, qr) = (pick(c, 1.0), pick(p, scale), pick(q, scale));
            let (t0r, t1r) = (pick(t0, 1.0), pick(t1, 1.0));
            let (slo, shi) = reference(cr, pr, qr, t0r, t1r);
            let slack = 4.0 * f64::EPSILON * (cr.abs() + pr.hypot(qr));
            assert!(
                lo <= slo + slack && hi >= shi - slack,
                "w = {w:e}, c = {c}, p = {p}, q = {q}, t = [{t0}, {t1}]; member \
                 ({cr}, {pr}, {qr}, [{t0r}, {t1r}]): window [{lo}, {hi}] does not contain \
                 [{slo}, {shi}] — {}",
                fuzz::replay()
            );
        }
    }
}

/// **Exact at `f64`, both sides.** Against the reference, the window is
/// off by no more than the rounding of its terms, `|c| + A`, plus what
/// rounding the span's middle and half-width (`ε·|t|` of angle) can
/// cost a misjudged crest test, at most `A` times that angle
/// ([`ArcWindow`]'s invariant).
#[test]
fn the_f64_window_is_the_extent_to_rounding() {
    let mut rng = fuzz::start("arc_window::f64_exact");
    for _ in 0..fuzz::scaled(20_000) {
        let (c, p, q, _) = harmonic(&mut rng);
        let (t0, t1) = span(&mut rng, q.atan2(p));
        let (lo, hi) = ArcWindow::new(t0, t1).extent(c, p, q);
        let (slo, shi) = reference(c, p, q, t0, t1);
        let a = p.hypot(q);
        let slack =
            16.0 * f64::EPSILON * (c.abs() + a) + 4.0 * f64::EPSILON * a * (t0.abs() + t1.abs());
        assert!(
            (lo - slo).abs() <= slack && (hi - shi).abs() <= slack,
            "c = {c}, p = {p}, q = {q}, t = [{t0}, {t1}]: window [{lo}, {hi}] vs \
             [{slo}, {shi}] (slack {slack:e}) — {}",
            fuzz::replay()
        );
    }
}

/// A random circle or ellipse: centre within 5 of the origin or far
/// out, a random plane and reference direction, radius from a
/// thousandth to a thousand, the ellipse as flat as `10⁻⁴`.
fn carrier(rng: &mut fuzz::Rng) -> Curve3<f64> {
    let far = if rng.below(4) == 0 { 1e3 } else { 1.0 };
    let mut coord = || rng.range(-5.0, 5.0) * far;
    let center = Point3::new(coord(), coord(), coord());
    let axis = Vec3::new(
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
    )
    .normalize();
    let (b0, b1) = axis.orthonormal_basis();
    let phi = rng.range(-PI, PI);
    let u_ref = b0 * phi.cos() + b1 * phi.sin();
    let semi_u = rng.log_uniform(1e-3, 1e3);
    if rng.below(2) == 0 {
        Curve3::Circle {
            center,
            axis,
            radius: semi_u,
            u_ref,
        }
    } else {
        Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v: semi_u * rng.log_uniform(1e-4, 1.0),
            u_ref,
        }
    }
}

/// `(c, p, q)` of `carrier` along `n` about the origin, and the scale
/// its terms round at, `(|centre| + the larger semi-axis)·|n|` — `c`
/// itself can cancel far below it.
fn harmonic_of(carrier: &Curve3<f64>, n: Vec3<f64>) -> (f64, f64, f64, f64) {
    let (center, axis, u, a, b) = match *carrier {
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, u_ref, radius, radius),
        Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => (center, axis, u_ref, semi_u, semi_v),
        _ => unreachable!("the sweep draws circles and ellipses"),
    };
    let v = axis.cross(u);
    (
        (center - Point3::origin()).dot(n),
        a * u.dot(n),
        b * v.dot(n),
        ((center - Point3::origin()).norm() + a.max(b)) * n.norm(),
    )
}

/// **Past a full turn every crest is in the span.** Spans of one to
/// four turns, either way, two in three centred so the crest or the
/// trough stands antipodal to the middle, where `cos h` and the
/// crest's cosine against the middle tie: both doors must read
/// `c ± A`, the f64 door to rounding and the certified door as an
/// enclosure no wider than rounding.
#[test]
fn past_a_full_turn_both_doors_read_both_crests() {
    let mut rng = fuzz::start("arc_window::past_a_turn");
    for _ in 0..fuzz::scaled(4_000) {
        let arc = carrier(&mut rng);
        let n = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        ) * rng.log_uniform(1e-2, 1e3);
        let (c, p, q, terms) = harmonic_of(&arc, n);
        let a = p.hypot(q);
        let len = rng.range(TAU, 4.0 * TAU);
        let middle = match rng.below(3) {
            0 => q.atan2(p) + PI,
            1 => q.atan2(p),
            _ => rng.range(-20.0, 20.0),
        };
        let (t0, t1) = (middle - len / 2.0, middle + len / 2.0);
        let (t0, t1) = if rng.below(2) == 0 {
            (t0, t1)
        } else {
            (t1, t0)
        };
        let tol = 16.0 * f64::EPSILON * terms;
        let (lo, hi) = conic_arc_extent_along(&arc, t0, t1, Point3::origin(), n).unwrap();
        assert!(
            (lo - (c - a)).abs() <= tol && (hi - (c + a)).abs() <= tol,
            "{arc:?} along {n:?} over [{t0}, {t1}]: f64 door [{lo}, {hi}] vs [{}, {}] — {}",
            c - a,
            c + a,
            fuzz::replay()
        );
        let (blo, bhi) =
            conic_arc_extent_along_bracketed(&arc, t0, t1, Point3::origin(), n).unwrap();
        assert!(
            blo <= c - a + tol && bhi >= c + a - tol && blo >= c - a - tol && bhi <= c + a + tol,
            "{arc:?} along {n:?} over [{t0}, {t1}]: certified door [{blo}, {bhi}] vs \
             [{}, {}] — {}",
            c - a,
            c + a,
            fuzz::replay()
        );
    }
}

/// **The box doors and the directional doors contain hard arcs.** A
/// 5000:1 and a 10⁸:1 ellipse in a tilted plane, a near-full turn, a
/// micron circle a million out, a descending span past a turn and an
/// exact half-turn: every sample of the arc lies in its box and in
/// both doors' extents along two oblique directions, up to the
/// sample's own rounding.
#[test]
fn the_box_and_directional_doors_contain_hard_arcs() {
    let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
    let u = axis.orthonormal_basis().0;
    let flat = |semi_v: f64| Curve3::Ellipse {
        center: Point3::new(-1.0, 0.5, 2.0),
        axis,
        semi_u: 5.0,
        semi_v,
        u_ref: u,
    };
    let circle = |center: Point3<f64>, radius: f64| Curve3::Circle {
        center,
        axis,
        radius,
        u_ref: u,
    };
    let cases = [
        ("flat 5000", flat(1e-3), 0.3, 5.9),
        ("flat 1e8", flat(5e-8), 0.3, 5.9),
        (
            "near full",
            circle(Point3::new(0.3, -0.2, 0.15), 1.0),
            0.1,
            0.1 + 0.999 * TAU,
        ),
        (
            "micron far",
            circle(Point3::new(1e6, -5e5, 2.5e5), 1e-6),
            0.7,
            2.9,
        ),
        (
            "descending past a turn",
            circle(Point3::new(0.0, 0.0, -3.0), 2.5),
            9.0,
            0.37 - TAU,
        ),
        (
            "exactly a half-turn",
            circle(Point3::new(0.0, 1.0, -3.0), 2.5),
            0.37,
            0.37 + PI,
        ),
    ];
    const SAMPLES: u32 = 20_000;
    let at = |t0: f64, t1: f64, i: u32| t0 + (t1 - t0) * f64::from(i) / f64::from(SAMPLES);
    for (what, c, t0, t1) in cases {
        let b = match c {
            Curve3::Circle { .. } => circle_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1)),
            _ => ellipse_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1)),
        }
        .unwrap();
        for i in 0..=SAMPLES {
            let (t, p) = (at(t0, t1, i), c.eval(at(t0, t1, i)));
            let s = 4.0 * f64::EPSILON * (1.0 + p.x.abs().max(p.y.abs()).max(p.z.abs()));
            for (w, lo, hi) in [
                (p.x, b.min_x, b.max_x),
                (p.y, b.min_y, b.max_y),
                (p.z, b.min_z, b.max_z),
            ] {
                assert!(
                    w >= lo - s && w <= hi + s,
                    "{what}: the box [{lo}, {hi}] drops the sample {w} at t = {t}"
                );
            }
        }
        for n in [Vec3::new(0.3, -0.8, 0.5), Vec3::new(-0.9, 0.1, 0.4)] {
            let o = Point3::origin();
            let (blo, bhi) = conic_arc_extent_along_bracketed(&c, t0, t1, o, n).unwrap();
            let (flo, fhi) = conic_arc_extent_along(&c, t0, t1, o, n).unwrap();
            for i in 0..=SAMPLES {
                let r = (c.eval(at(t0, t1, i)) - o).dot(n);
                let s = 8.0 * f64::EPSILON * (1.0 + r.abs());
                assert!(
                    r >= blo - s && r <= bhi + s,
                    "{what}: the certified door [{blo}, {bhi}] along {n:?} drops the sample {r}"
                );
                assert!(
                    r >= flo - s && r <= fhi + s,
                    "{what}: the f64 door [{flo}, {fhi}] along {n:?} drops the sample {r}"
                );
            }
        }
    }
}

/// **An undecided tie at `h ≈ π` hulls.** A span of one turn less (or
/// more) than its brackets' width, starting and ending at the crest:
/// the bracketed span holds both full turns and turns short of one, so
/// the window cannot tell whether the crest is in, and must read the
/// hull — the crest `1 + A` and the trough `1 − A` both inside.
#[test]
fn an_undecided_tie_at_a_full_turn_takes_both_crests() {
    let (p, q) = (0.6_f64, 0.8_f64);
    let (a, ts) = (p.hypot(q), q.atan2(p));
    for w in [1e-12, 1e-6, 1e-2] {
        for (t0, t1) in [
            (ts + w / 2.0, ts + TAU - w / 2.0),
            (ts + TAU - w / 2.0, ts + w / 2.0),
        ] {
            let wid = |x: f64| Interval::from_bounds(x - w, x + w);
            let window = ArcWindow::new(wid(t0), wid(t1));
            let tie = window.crest_outside(wid(p), wid(q));
            assert!(
                tie.lo() <= 0.0 && tie.hi() > 0.0,
                "w = {w:e}, [{t0}, {t1}]: the crest test [{}, {}] is decided, so this row \
                 does not reach the hull",
                tie.lo(),
                tie.hi()
            );
            let (lo, hi) = window.extent(wid(1.0), wid(p), wid(q));
            assert!(
                hi.hi() >= 1.0 + a && lo.lo() <= 1.0 - a,
                "w = {w:e}, [{t0}, {t1}]: [{}, {}] drops a crest of [{}, {}]",
                lo.lo(),
                hi.hi(),
                1.0 - a,
                1.0 + a
            );
        }
    }
}

/// **Past a turn, the crest antipodal to the span's middle is in.** The
/// span's half-width exceeds π, so the crest at `m + π` is one the
/// span holds; the cosine test alone ties there (`cos h` at its floor
/// of −1 against the crest's own −1), and rounding read it as outside,
/// returning an end about 65 short of `A ≈ 1055`.
#[test]
fn past_a_turn_the_f64_door_holds_the_crest_antipodal_to_the_middle() {
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let n = Vec3::new(730.3003055298171, 761.676099383592, 0.0);
    let (t0, t1) = (0.4539818544898191, 7.442052950951721);
    let a = n.norm();
    let tol = 1e-12 * a;
    let o = Point3::origin();
    let (lo, hi) = conic_arc_extent_along(&circle, t0, t1, o, n).unwrap();
    assert!(
        (hi - a).abs() <= tol && (lo + a).abs() <= tol,
        "f64 door [{lo}, {hi}] vs [-{a}, {a}]"
    );
    let (blo, bhi) = conic_arc_extent_along_bracketed(&circle, t0, t1, o, n).unwrap();
    assert!(
        blo <= -a && bhi >= a && blo >= -a - tol && bhi <= a + tol,
        "certified door [{blo}, {bhi}] vs [-{a}, {a}]"
    );
}
