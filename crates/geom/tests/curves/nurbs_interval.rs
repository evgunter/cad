//! M5 PR 3 acceptance, interval lane (spec row 4): Interval
//! containment for NURBS curve evaluation — single-span boxes and
//! knot-straddling boxes — plus poison propagation and the
//! `Dual<Interval>` instantiation.
//!
//! Containment logic: every interval ring op encloses all exact
//! results over contained inputs and rounds outward, and f64's
//! nearest-rounded step result stays inside the outward-rounded
//! enclosure of a contained intermediate — so an f64 run of the same
//! fixed-association recipe is contained inductively, and asserting
//! `eval(Interval) ∋ eval(f64)` is legitimate here (pure ring
//! arithmetic; contrast the transcendental caveat in `Bounds`' docs).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::needless_range_loop)]

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Dual, Interval, Point3, Real};

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// The §7.3 rational-quadratic full circle in a tilted exact frame
/// (the differential suite's fixture, duplicated — integration tests
/// are separate crates).
fn nurbs_circle() -> NurbsCurve3<f64> {
    let c = Point3::new(-0.5, 4.0, 1.25);
    let x = geom_core::Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);
    let y = geom_core::Vec3::new(2.0 / 3.0, -1.0 / 3.0, -2.0 / 3.0);
    let r = 2.5;
    let p = |cx: f64, cy: f64| c + x * (r * cx) + y * (r * cy);
    let control = vec![
        p(1.0, 0.0),
        p(1.0, 1.0),
        p(0.0, 1.0),
        p(-1.0, 1.0),
        p(-1.0, 0.0),
        p(-1.0, -1.0),
        p(0.0, -1.0),
        p(1.0, -1.0),
        p(1.0, 0.0),
    ];
    let weights = vec![1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0];
    let knots = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    NurbsCurve3::new(knots, control, weights).unwrap()
}

fn contains(e: Interval, v: f64) -> bool {
    e.lo() <= v && v <= e.hi()
}

/// Single-span boxes: `eval(t: Interval)` contains `eval(t: f64)` for
/// a dense grid of f64 parameters inside the box.
#[test]
fn interval_eval_contains_f64_eval_single_span() {
    let n = nurbs_circle();
    let ni = n.map_scalar(Interval::from_f64);
    for (lo, hi) in [(0.02, 0.2), (0.26, 0.49), (0.51, 0.74), (0.9, 0.999)] {
        let t = Interval::from_bounds(lo, hi);
        let p = ni.eval(t);
        for i in 0..=60usize {
            let tf = lo + (hi - lo) * i as f64 / 60.0;
            let pf = n.eval(tf);
            assert!(
                contains(p.x, pf.x) && contains(p.y, pf.y) && contains(p.z, pf.z),
                "eval containment fails on [{lo}, {hi}] at t = {tf}"
            );
        }
        // Derivatives too.
        let d = ni.deriv(t);
        let d2 = ni.deriv2(t);
        for i in 0..=20usize {
            let tf = lo + (hi - lo) * i as f64 / 20.0;
            let df = n.deriv(tf);
            let d2f = n.deriv2(tf);
            assert!(contains(d.x, df.x) && contains(d.y, df.y) && contains(d.z, df.z));
            assert!(contains(d2.x, d2f.x) && contains(d2.y, d2f.y) && contains(d2.z, d2f.z));
        }
    }
}

/// Knot-straddling boxes: the hull-of-spans result still contains
/// every pointwise value (continuity across the knot).
#[test]
fn interval_eval_contains_f64_across_knots() {
    let n = nurbs_circle();
    let ni = n.map_scalar(Interval::from_f64);
    for (lo, hi) in [(0.2, 0.3), (0.45, 0.8), (0.0, 1.0)] {
        let t = Interval::from_bounds(lo, hi);
        let p = ni.eval(t);
        let d = ni.deriv(t);
        for i in 0..=120usize {
            let tf = lo + (hi - lo) * i as f64 / 120.0;
            let pf = n.eval(tf);
            let df = n.deriv(tf);
            assert!(
                contains(p.x, pf.x) && contains(p.y, pf.y) && contains(p.z, pf.z),
                "straddle containment fails on [{lo}, {hi}] at t = {tf}"
            );
            assert!(contains(d.x, df.x) && contains(d.y, df.y) && contains(d.z, df.z));
        }
    }
}

/// Poison propagation (spec row 4): poisoned parameter or control
/// point yields a poisoned result — a value, never a decision, never
/// a panic.
#[test]
fn poison_propagates_through_values() {
    let n = nurbs_circle();
    let ni = n.map_scalar(Interval::from_f64);
    // Poisoned parameter (NaI).
    let p = ni.eval(Interval::from_f64(f64::NAN));
    // Every channel, not the first: NaI in poisons the whole point.
    assert!(p.x.is_poison() && p.y.is_poison() && p.z.is_poison());
    // Poisoned control point at f64.
    let mut ctrl = n.control().to_vec();
    ctrl[3] = Point3::new(f64::NAN, 0.0, 0.0);
    let np = NurbsCurve3::new(n.knots_f64_interim().clone(), ctrl, n.weights().to_vec()).unwrap();
    // A parameter whose span touches control index 3 poisons the
    // poisoned CHANNEL and leaves the others finite — the partial
    // answer, stated as what it is rather than checked on x alone.
    let near = np.eval(0.3);
    assert!(near.x.is_nan() && near.y.is_finite() && near.z.is_finite());
    // A far-away span does not (locality of the basis) — on every
    // channel, so the claim cannot be met by a net poisoned elsewhere.
    let far = np.eval(0.9);
    assert!(far.x.is_finite() && far.y.is_finite() && far.z.is_finite());
}

/// `Dual<Interval>` instantiates and both channels contain the
/// pointwise `Dual<f64>` runs (value and derivative channels hulled
/// independently across spans — the seam's convention).
#[test]
fn dual_interval_channels_contain_pointwise_duals() {
    let n = nurbs_circle();
    let ndi: NurbsCurve3<Dual<Interval>> = n.map_scalar(|x| Dual::constant(Interval::from_f64(x)));
    for (lo, hi) in [(0.1, 0.2), (0.45, 0.55)] {
        let t = Dual::variable(Interval::from_bounds(lo, hi));
        let p = ndi.eval(t);
        for i in 0..=40usize {
            let tf = lo + (hi - lo) * i as f64 / 40.0;
            let pf = n.eval(tf);
            let df = n.deriv(tf);
            assert!(
                contains(p.x.value, pf.x),
                "value channel at t = {tf} in [{lo}, {hi}]"
            );
            assert!(
                contains(p.x.deriv, df.x),
                "deriv channel at t = {tf} in [{lo}, {hi}]"
            );
            assert!(contains(p.y.deriv, df.y) && contains(p.z.deriv, df.z));
        }
    }
}

/// Whether a point evaluation's enclosure meets the door's: the true
/// value lies in both, so an empty meet proves the door misses it.
fn meets(x: Interval, b: Interval) -> bool {
    b.lo() <= x.hi() && x.lo() <= b.hi()
}

/// **`span_enclosure` and `span_deriv` enclose the curve and its
/// derivative over a parameter range**: ranges inside one span, ranges
/// straddling the circle's double knots, and the whole domain, each
/// sampled densely; every `C(t)` and `C′(t)` evaluated at `Interval`
/// at a point parameter meets the door's box. A range with no extent,
/// or one past the domain, is declined.
#[test]
fn span_enclosure_and_span_deriv_contain_every_sampled_value() {
    let ni = nurbs_circle().map_scalar(Interval::from_f64);
    let ranges = [
        (0.05, 0.2),
        (0.2, 0.3),
        (0.24, 0.26),
        (0.1, 0.6),
        (0.49, 0.5),
        (0.7, 0.9999),
        (0.0, 1.0),
        (0.3, 0.3 + 1e-9),
    ];
    for (lo, hi) in ranges {
        let enc = ni
            .span_enclosure(lo, hi)
            .expect("a range inside the domain");
        let der = ni.span_deriv(lo, hi).expect("a range inside the domain");
        for k in 0..=64 {
            let t = (lo + (hi - lo) * f64::from(k) / 64.0).min(hi);
            let p = ni.eval(Interval::from_f64(t));
            let d = ni.deriv(Interval::from_f64(t));
            assert!(
                meets(p.x, enc.x) && meets(p.y, enc.y) && meets(p.z, enc.z),
                "C({t}) = {p:?} misses span_enclosure({lo}, {hi}) = {enc:?}"
            );
            assert!(
                meets(d.x, der.x) && meets(d.y, der.y) && meets(d.z, der.z),
                "C′({t}) = {d:?} misses span_deriv({lo}, {hi}) = {der:?}"
            );
        }
    }
    for (lo, hi) in [(0.4, 0.4), (0.6, 0.5), (-0.1, 0.5), (0.5, 1.1)] {
        assert!(
            ni.span_enclosure(lo, hi).is_none() && ni.span_deriv(lo, hi).is_none(),
            "{lo}..{hi} is declined"
        );
    }
    // **And they NARROW**, which containment alone cannot see: the whole
    // line contains every sample. A range inside another has an
    // enclosure no wider on any axis, and a range a millionth of the
    // domain long encloses the curve within a few millionths of its
    // radius (2.5) — the curve's speed bounds its motion over the range.
    let width = |p: Point3<Interval>| {
        [p.x, p.y, p.z]
            .map(|c| c.hi() - c.lo())
            .into_iter()
            .fold(0.0f64, f64::max)
    };
    let (outer, inner) = (
        ni.span_enclosure(0.1, 0.6).unwrap(),
        ni.span_enclosure(0.2, 0.3).unwrap(),
    );
    assert!(
        width(inner) <= width(outer) && width(outer).is_finite(),
        "a sub-range encloses no wider: {inner:?} within {outer:?}"
    );
    let tiny = ni.span_enclosure(0.3, 0.3 + 1e-6).unwrap();
    assert!(width(tiny) < 1e-4, "a millionth of the domain: {tiny:?}");
    let speed = |d: geom_core::Vec3<Interval>| {
        [d.x, d.y, d.z]
            .map(|c| c.lo().abs().max(c.hi().abs()))
            .into_iter()
            .fold(0.0f64, f64::max)
    };
    let (d_outer, d_inner) = (
        ni.span_deriv(0.1, 0.6).unwrap(),
        ni.span_deriv(0.2, 0.3).unwrap(),
    );
    assert!(
        speed(d_outer).is_finite() && speed(d_inner) <= speed(d_outer),
        "the hodograph narrows with the range: {d_inner:?} within {d_outer:?}"
    );
    // The circle's speed is bounded on a quarter's span: a hodograph
    // enclosure tens of radii wide would be the whole line by another
    // name.
    assert!(speed(d_inner) < 100.0, "{d_inner:?}");
}

/// **A range's few-ulp meet with the span past a knot narrows like any
/// other.** A cubic with a simple knot at `k = ½ − 2 ulp` and
/// coordinates no float holds exactly; the range `[¼, ½]` meets the span
/// past `k` over two ulps. Every sampled `C′` meets `span_deriv`, and
/// its enclosure is no wider than a hundredth of the curve's speed
/// there: red if the derivative were read by differencing the meet's
/// own two-ulp restriction, whose rounding over its width is slope.
#[test]
fn span_deriv_narrows_on_a_few_ulp_meet_past_a_knot() {
    let k = 0.5f64.next_down().next_down();
    let p = |x: f64, y: f64| Point3::new(0.3 + 0.1 * x, 0.7 + 0.1 * y, 0.1 * x * y);
    let control = vec![
        p(0.0, 0.0),
        p(1.0, 0.5),
        p(2.0, 1.5),
        p(3.0, 1.0),
        p(4.0, 2.5),
    ];
    let ni = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, k, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        control,
        vec![1.0; 5],
    )
    .unwrap()
    .map_scalar(Interval::from_f64);
    let (lo, hi) = (0.25, 0.5);
    let der = ni.span_deriv(lo, hi).expect("a range inside the domain");
    for t in [lo, 0.375, k, hi] {
        let d = ni.deriv(Interval::from_f64(t));
        assert!(
            meets(d.x, der.x) && meets(d.y, der.y) && meets(d.z, der.z),
            "C′({t}) = {d:?} misses span_deriv({lo}, {hi}) = {der:?}"
        );
    }
    let at_knot = ni.deriv(Interval::from_f64(k));
    let speed = [at_knot.x, at_knot.y, at_knot.z]
        .map(|c| c.lo().abs().max(c.hi().abs()))
        .into_iter()
        .fold(0.0f64, f64::max);
    let ranged = ni.span_deriv(lo, k).expect("the near span alone");
    for (c, near) in [(der.x, ranged.x), (der.y, ranged.y), (der.z, ranged.z)] {
        assert!(
            c.hi() - c.lo() <= (near.hi() - near.lo()) + speed / 100.0,
            "the two-ulp meet widens span_deriv({lo}, {hi}) = {der:?} past the near span's \
             {ranged:?}"
        );
    }
}

/// **`span_deriv` does not move with the curve.** The circle and its
/// twin moved by `(1e3, −1e3, 1e3)`, over ranges from a quarter to
/// `2⁻³⁰`: each coordinate's width matches its twin's within 1% plus a
/// rounding allowance of `1e-9`. Read in the absolute frame, the moved
/// circle's derivative carries the distance times the weight's slope.
#[test]
fn span_deriv_does_not_move_with_the_curve() {
    let home = nurbs_circle();
    let by = geom_core::Vec3::new(1e3, -1e3, 1e3);
    let away = home.map_points(|p| p + by).map_scalar(Interval::from_f64);
    let home = home.map_scalar(Interval::from_f64);
    let width = |d: geom_core::Vec3<Interval>| [d.x, d.y, d.z].map(|c| c.hi() - c.lo());
    for k in [2, 5, 10, 20, 30] {
        let h = 2f64.powi(-k);
        for lo in [0.0, 0.1, 0.3, 0.5 - h, 0.6] {
            let (a, b) = (
                width(home.span_deriv(lo, lo + h).unwrap()),
                width(away.span_deriv(lo, lo + h).unwrap()),
            );
            for c in 0..3 {
                assert!(
                    b[c] <= 1.01 * a[c] + 1e-9 && a[c] <= 1.01 * b[c] + 1e-9,
                    "C′[{c}] over [{lo}, {}]: {} wide moved against {} at home",
                    lo + h,
                    b[c],
                    a[c]
                );
            }
        }
    }
}

/// **`span_deriv` is tight against the curve's own turning, off the
/// origin.** The circle moved by `(1e3, −1e3, 1e3)`, over ranges `2⁻²` to
/// `2⁻³⁰` long, some straddling its double knots: the widest coordinate
/// of the enclosure is at most 4× that coordinate's spread over 33
/// sampled points of the range, plus `1e-9`. A range read about a point
/// far from its piece carries that distance times the weight's slope,
/// which shrinks no faster than the range's own spread.
#[test]
fn span_deriv_is_tight_against_its_own_range_off_the_origin() {
    let by = geom_core::Vec3::new(1e3, -1e3, 1e3);
    let away = nurbs_circle()
        .map_points(|p| p + by)
        .map_scalar(Interval::from_f64);
    let mut worst = 0.0f64;
    for k in [2, 4, 8, 16, 30] {
        let h = 2f64.powi(-k);
        for lo in [0.0, 0.1, 0.25 - h / 2.0, 0.6, 1.0 - h] {
            let d = away.span_deriv(lo, lo + h).unwrap();
            let samples: Vec<_> = (0..=32)
                .map(|i| {
                    let t = (lo + h * f64::from(i) / 32.0).min(lo + h);
                    away.deriv(Interval::from_f64(t))
                })
                .collect();
            let spread = |pick: fn(&geom_core::Vec3<Interval>) -> Interval| {
                let lo = samples
                    .iter()
                    .map(|d| pick(d).hi())
                    .fold(f64::INFINITY, f64::min);
                let hi = samples
                    .iter()
                    .map(|d| pick(d).lo())
                    .fold(f64::NEG_INFINITY, f64::max);
                (hi - lo).max(0.0)
            };
            let spread = spread(|d| d.x).max(spread(|d| d.y)).max(spread(|d| d.z));
            let width = [d.x, d.y, d.z]
                .iter()
                .map(|c| c.hi() - c.lo())
                .fold(0.0, f64::max);
            worst = worst.max(width / spread.max(1e-300));
            assert!(
                width <= 4.0 * spread + 1e-9,
                "C′ over [{lo}, {}]: {width} wide against a sampled spread of {spread}",
                lo + h
            );
        }
    }
    println!("[jet frame] the circle off the origin: worst width / spread {worst}");
}

/// **A memoised edge answers each range as the plain edge does, bit for
/// bit.** The circle's domain halved eight levels deep, read parent
/// first, so a range is asked after the range that holds it and beside
/// its sibling: the memo's `span_enclosure` and `span_deriv` are the
/// plain edge's, on the first read and on the second, which the memo
/// serves. A memo keyed by less than the whole range serves a parent's
/// or a sibling's answer here.
#[test]
fn a_memoised_edge_restricts_each_range_as_the_plain_edge_does() {
    let plain = nurbs_circle().map_scalar(Interval::from_f64);
    let memo = plain.with_span_memo();
    let bits = |cs: [Interval; 3]| cs.map(|c| [c.lo().to_bits(), c.hi().to_bits()]);
    let mut level = vec![(0.0, 1.0)];
    for depth in 0..8 {
        for &(lo, hi) in &level {
            let want_enc = plain.span_enclosure(lo, hi).map(|p| bits([p.x, p.y, p.z]));
            let want_der = plain.span_deriv(lo, hi).map(|d| bits([d.x, d.y, d.z]));
            assert!(
                want_enc.is_some() && want_der.is_some(),
                "depth {depth}: the range {lo}..{hi} is inside the domain"
            );
            for pass in ["first", "second"] {
                assert_eq!(
                    memo.span_enclosure(lo, hi).map(|p| bits([p.x, p.y, p.z])),
                    want_enc,
                    "depth {depth}, {pass} read: the memo's enclosure of {lo}..{hi}"
                );
                assert_eq!(
                    memo.span_deriv(lo, hi).map(|d| bits([d.x, d.y, d.z])),
                    want_der,
                    "depth {depth}, {pass} read: the memo's derivative over {lo}..{hi}"
                );
            }
        }
        level = level
            .iter()
            .flat_map(|&(lo, hi)| {
                let m = 0.5 * (lo + hi);
                [(lo, m), (m, hi)]
            })
            .collect();
    }
}
