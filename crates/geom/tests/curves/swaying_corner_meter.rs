//! **The integral speed meter answers on a curve that sways across its
//! chords, and stays a lower bound.** A corner path that swings side to
//! side has derivative coefficients pointing back along every control
//! chord, so the global and per-span chord assemblies go negative on a
//! curve that never stalls; the piece assembly reads the derivative's
//! Bernstein coefficients on thirty-two pieces per span, which converge on
//! `C′` itself.
//!
//! The reference every row compares against is [`min_speed_ceiling`]:
//! the least UPPER end of an interval enclosure of `‖C′(t)‖` over a
//! dense grid and a golden-section refinement of each grid minimum. Each
//! such end is at least the true speed at its `t`, so the reference is
//! at least the true minimum speed, and a meter above it is unsound
//! with certainty — no sampling tolerance enters the verdict.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// `!(m > x)` is deliberate: a NaN meter is an abstention, never a pass
// of a "must refuse" row.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

test_utils::gated_to![
    "crates/geom/src/curves/",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/spline/",
];

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};
use test_utils::fuzz;

/// An upper bound on the true minimum of `‖C′‖` over the domain: the
/// least enclosure ceiling over `n + 1` grid points and a 60-step
/// golden-section refinement around every grid-local minimum.
fn min_speed_ceiling(c: &NurbsCurve3<f64>, n: usize) -> f64 {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    let ceiling = |t: f64| ci.deriv(Interval::from_f64(t)).norm().hi();
    let speed = |t: f64| c.deriv(t).norm();
    let (lo, hi) = c.domain();
    #[allow(clippy::cast_precision_loss)]
    let ts: Vec<f64> = (0..=n)
        .map(|i| lo + (hi - lo) * (i as f64) / (n as f64))
        .collect();
    let ss: Vec<f64> = ts.iter().map(|t| speed(*t)).collect();
    let mut best = ts.iter().map(|t| ceiling(*t)).fold(f64::INFINITY, f64::min);
    for i in 0..=n {
        let left = if i == 0 { f64::INFINITY } else { ss[i - 1] };
        let right = if i == n { f64::INFINITY } else { ss[i + 1] };
        if ss[i] > left || ss[i] > right {
            continue;
        }
        let (mut a, mut b) = (ts[i.saturating_sub(1)], ts[(i + 1).min(n)]);
        let g = geom_core::GOLDEN_SECTION;
        for _ in 0..60 {
            let (x, y) = (b - g * (b - a), a + g * (b - a));
            if speed(x) < speed(y) {
                b = y;
            } else {
                a = x;
            }
        }
        best = best.min(ceiling(0.5 * (a + b)));
    }
    best
}

/// The meter at both scalars: the `f64` reading and the certified
/// lane's floor.
fn meters(c: &NurbsCurve3<f64>) -> (f64, f64) {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    (
        c.speed_lower_bound().get(),
        ci.speed_lower_bound().get().lo(),
    )
}

/// The swaying corner: `(±a, 0, z)` alternating through z = 0, 1, 2, 3,
/// interpolated at `degree` (the shape of a wavy loft's corner edge).
fn swaying(a: f64, degree: usize) -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = [-a, a, -a, a]
        .into_iter()
        .zip([0.0, 1.0, 2.0, 3.0])
        .map(|(x, z)| Point3::new(x, 0.0, z))
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, degree).unwrap()
}

/// The swaying corners answer positive at both scalars, and below the
/// reference. The floor is under the measured tightness on this family
/// (`m / ceiling` from 0.998 at `a = 0.75` to 0.55 at degree 3,
/// `a = 4`): a schedule change that gives most of it back is visible
/// here.
#[test]
fn a_swaying_corner_meters_positive_and_below_its_speed() {
    for (degree, a) in [
        (2, 0.75),
        (2, 1.0),
        (2, 1.5),
        (2, 2.0),
        (2, 4.0),
        (3, 0.5),
        (3, 1.0),
        (3, 2.0),
        (3, 4.0),
    ] {
        let c = swaying(a, degree);
        let reference = min_speed_ceiling(&c, 4000);
        let (m, mi) = meters(&c);
        assert!(m > 0.0 && mi > 0.0, "degree {degree}, a = {a}: {m}, {mi}");
        assert!(
            m <= reference && mi <= reference,
            "degree {degree}, a = {a}: meter {m} / {mi} above the speed {reference}"
        );
        assert!(
            m >= 0.5 * reference,
            "degree {degree}, a = {a}: meter {m} gives away half of {reference}"
        );
    }
}

/// [`swaying`] with its weights replaced by `weights(j)` (same knots,
/// same net).
fn reweighted(c: &NurbsCurve3<f64>, weights: impl Fn(usize) -> f64) -> NurbsCurve3<f64> {
    let w = (0..c.control().len()).map(weights).collect();
    NurbsCurve3::<f64>::new(c.knots_f64_interim().clone(), c.control().to_vec(), w).unwrap()
}

/// The swaying family the rational meter is pinned on: the integral
/// rows' far end, where the chord assemblies are long gone.
const RATIONAL_FAMILY: [(usize, f64); 4] = [(2, 4.0), (3, 0.5), (3, 2.0), (3, 4.0)];

/// **Equal weights are polynomial.** A curve whose weights are all one
/// value `w ≠ 1` is the polynomial curve on the same net, and meters
/// exactly as it does, bit for bit at both scalars.
#[test]
fn a_uniform_weight_curve_meters_as_its_polynomial() {
    for (degree, a) in RATIONAL_FAMILY {
        let c = swaying(a, degree);
        let (m, mi) = meters(&c);
        for w in [0.7, 3.0] {
            let (mw, mwi) = meters(&reweighted(&c, |_| w));
            assert!(
                mw.to_bits() == m.to_bits() && mwi.to_bits() == mi.to_bits(),
                "degree {degree}, a = {a}, weights all {w}: metered {mw} / {mwi}, \
                 its polynomial {m} / {mi}"
            );
        }
    }
}

/// **A swaying rational corner meters.** With one interior weight
/// raised 1.3× — a genuinely rational curve — the swaying family
/// answers positive at both scalars and below the enclosure reference.
/// The floor is under the measured tightness (`m / ceiling` from 0.99
/// at degree 3, `a = 0.5` to 0.50 at degree 3, `a = 4`), so a schedule
/// change that gives most of it back shows.
#[test]
fn a_swaying_rational_corner_meters_positive_and_below_its_speed() {
    for (degree, a) in RATIONAL_FAMILY {
        let c = reweighted(&swaying(a, degree), |j| if j == 1 { 1.3 } else { 1.0 });
        let reference = min_speed_ceiling(&c, 4000);
        let (m, mi) = meters(&c);
        let at = format!("degree {degree}, a = {a}, w₁ = 1.3");
        assert!(m > 0.0 && mi > 0.0, "{at}: {m}, {mi}");
        assert!(
            m <= reference && mi <= reference,
            "{at}: meter {m} / {mi} above the speed {reference}"
        );
        assert!(
            m >= 0.25 * reference,
            "{at}: meter {m} gives away most of {reference}"
        );
    }
}

/// **The arm choice is monotone in reach.** An equal-weight curve takes
/// the integral arm and the same net with one weight one ulp off takes
/// the rational arm; at the shared piece count the two read the same
/// bound (measured: the last ulp at `f64`, about `1e-12` relative at
/// `Interval`), so being exactly polynomial never meters worse than
/// `1e-9` relative below the rational reading.
#[test]
fn an_equal_weight_curve_meters_no_worse_than_one_ulp_off() {
    for (degree, a) in [(2, 2.0), (2, 4.0), (3, 2.0), (3, 4.0)] {
        let c = swaying(a, degree);
        let at = format!("degree {degree}, a = {a}");
        arms_agree(&c, &at);
    }
}

/// The integral arm on `c` reads no lower than `1e-9` relative below
/// the rational arm on `c` with one weight one ulp off, at both scalars.
fn arms_agree(c: &NurbsCurve3<f64>, at: &str) {
    let off = reweighted(c, |j| if j == 1 { 1.0 + f64::EPSILON } else { 1.0 });
    let (m, mi) = meters(c);
    let (r, ri) = meters(&off);
    assert!(
        m >= r - 1e-9 * r.abs() && mi >= ri - 1e-9 * ri.abs(),
        "{at}: equal weights meter {m} / {mi}, one ulp off {r} / {ri}"
    );
}

/// A random integral net, degree 1–5, on a random clamped vector with
/// some interior knots raised. With `stall`, a random interior knot is
/// raised to multiplicity `p − 1` (at least 1) and the two control
/// points whose difference is `C′` there are made equal, so `C′`
/// vanishes at that knot exactly in ℝ.
fn random_curve(
    r: &mut fuzz::Rng,
    deg: usize,
    npts: usize,
    stall: bool,
) -> Option<NurbsCurve3<f64>> {
    let mut control: Vec<Point3<f64>> = (0..npts)
        .map(|_| Point3::new(r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)))
        .collect();
    let interior = npts - deg - 1;
    let stall_mult = (deg - 1).max(1);
    if stall && (deg < 2 || interior < stall_mult) {
        return None;
    }
    let stall_at = if stall {
        r.below(interior - stall_mult + 1)
    } else {
        usize::MAX
    };
    let mut knots = vec![0.0; deg + 1];
    let mut v = 0.0;
    let mut i = 0;
    while i < interior {
        v += r.range(0.2, 1.2);
        let mult = if i == stall_at {
            // `u_k = … = u_{k+p−2}` with `k = deg + 1 + i`: the derivative
            // spline is interpolatory there, `C′(u) = Q_{k−2}`, and
            // `Q_{k−2} ∝ P_{k−1} − P_{k−2}`.
            let k = deg + 1 + i;
            control[k - 1] = control[k - 2];
            stall_mult
        } else if r.unit() < 0.2 {
            // Never stepping over the stall's knot.
            let room = if i < stall_at {
                stall_at - i
            } else {
                interior - i
            };
            (1 + r.below(deg)).min(room)
        } else {
            1
        };
        for _ in 0..mult {
            knots.push(v);
        }
        i += mult;
    }
    v += r.range(0.2, 1.2);
    knots.extend(std::iter::repeat_n(v, deg + 1));
    let kv = KnotVector::clamped(knots, deg).ok()?;
    NurbsCurve3::<f64>::new(kv, control, vec![1.0; npts]).ok()
}

/// **Soundness fuzz** (counterexample search): on random integral nets
/// and on nets that stall exactly, neither scalar's meter exceeds the
/// enclosure reference, and a stalling net never meters a positive
/// speed the gate could pass.
#[test]
fn the_meter_stays_below_the_speed_and_refuses_a_stall() {
    let mut rng = fuzz::start("swaying_corner_meter::sound");
    let cases = fuzz::scaled(60);
    let (mut built, mut stalls) = (0_usize, 0_usize);
    for case in 0..cases {
        let deg = 1 + case % 5;
        let npts = deg + 2 + rng.below(9);
        let stall = case % 3 == 0;
        let Some(c) = random_curve(&mut rng, deg, npts, stall) else {
            continue;
        };
        built += 1;
        let reference = min_speed_ceiling(&c, fuzz::scaled(200));
        let (m, mi) = meters(&c);
        assert!(
            !(m > reference) && !(mi > reference),
            "case {case} (deg {deg}, {npts} pts, stall {stall}): meter {m} / {mi} \
             above the speed ceiling {reference} — {}",
            fuzz::replay()
        );
        if stall {
            stalls += 1;
            assert!(
                !(mi > 0.0) && !(m > 1e-12),
                "case {case} (deg {deg}, {npts} pts): a stalling net metered {m} / {mi} — {}",
                fuzz::replay()
            );
        }
    }
    assert!(
        built * 2 >= cases && stalls * 8 >= cases,
        "fuzz rot: {built} nets, {stalls} stalls of {cases} — {}",
        fuzz::replay()
    );
}

/// **Rational soundness fuzz** (counterexample search): the integral
/// fuzz's nets with random positive weights (`0.2..5`), every third an
/// exact stall (equal control points at a knot of multiplicity
/// `p − 1`, where `A′w − Aw′` vanishes for any weights). Neither
/// scalar's meter exceeds the enclosure reference, and a stall never
/// meters positive. Every tenth net instead carries the swaying
/// table's two weight shapes, all `0.7` and one weight raised `1.3×`.
#[test]
fn the_rational_meter_stays_below_the_speed_and_refuses_a_stall() {
    let mut rng = fuzz::start("swaying_corner_meter::rational_sound");
    let cases = fuzz::scaled(60);
    let (mut built, mut stalls) = (0_usize, 0_usize);
    for case in 0..cases {
        let deg = 1 + case % 5;
        let npts = deg + 2 + rng.below(9);
        let stall = case % 3 == 0;
        let Some(c) = random_curve(&mut rng, deg, npts, stall) else {
            continue;
        };
        let raised = rng.below(npts);
        let c = match case % 10 {
            0 => reweighted(&c, |_| 0.7),
            5 => reweighted(&c, |j| if j == raised { 1.3 } else { 1.0 }),
            _ => {
                let w: Vec<f64> = (0..npts).map(|_| rng.range(0.2, 5.0)).collect();
                reweighted(&c, |j| w[j])
            }
        };
        built += 1;
        let reference = min_speed_ceiling(&c, fuzz::scaled(200));
        let (m, mi) = meters(&c);
        // The `f64` reading is a bound up to its stated rounding (a few
        // tens of relative ulps); the interval floor is held strictly.
        assert!(
            !(m > reference * (1.0 + 64.0 * f64::EPSILON)) && !(mi > reference),
            "case {case} (deg {deg}, {npts} pts, stall {stall}): meter {m} / {mi} \
             above the speed ceiling {reference} — {}",
            fuzz::replay()
        );
        if stall {
            stalls += 1;
            assert!(
                !(mi > 0.0) && !(m > 1e-12),
                "case {case} (deg {deg}, {npts} pts): a stalling net metered {m} / {mi} — {}",
                fuzz::replay()
            );
        }
    }
    assert!(
        built * 2 >= cases && stalls * 8 >= cases,
        "fuzz rot: {built} nets, {stalls} stalls of {cases} — {}",
        fuzz::replay()
    );
}

/// **The rational meter is tight where the truth is reachable.** On a
/// degree-1 rational net every span is a straight segment, `A′w − Aw′`
/// is constant on it, and each piece's `w` row ends on `w`'s value at
/// the piece's end, so the piece assembly's bound IS the true minimum
/// speed in ℝ (`w₀w₁‖ΔP‖/(Δu·w_max²)` on the slowest span). The
/// interval floor must therefore sit between the enclosure ceiling and
/// `1e-9` below it: an inflation of the bound by `1e-4` fails the upper
/// assertion, a loss of tightness the lower.
#[test]
fn a_degree_one_rational_net_meters_its_exact_minimum() {
    let segment = NurbsCurve3::<f64>::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 1.0, 0.0)],
        vec![1.0, 3.0],
    )
    .unwrap();
    // A zigzag that doubles back at each corner, weights spanning 0.5–3.
    let zigzag = NurbsCurve3::<f64>::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 2.5, 3.0, 3.0], 1).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 2.0, 0.0),
            Point3::new(0.5, -1.0, 1.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0, 3.0, 0.5, 2.0],
    )
    .unwrap();
    for (name, c) in [("segment", &segment), ("zigzag", &zigzag)] {
        let reference = min_speed_ceiling(c, 400);
        let (_, mi) = meters(c);
        assert!(
            mi <= reference && mi >= reference * (1.0 - 1e-9),
            "{name}: interval meter {mi} against the exact minimum's ceiling {reference} \
             (ratio {})",
            mi / reference
        );
    }
}

/// The meter is memoised on the curve, and the memo is not part of its
/// value: reading it leaves `Debug` unchanged, a clone reads the same
/// bits, and a curve built from a metered one by a door that moves the
/// net (here a uniform scale by 2, which doubles the speed) meters its
/// own net rather than inheriting the first answer. A symbolic session
/// is not served it either (the row below).
#[test]
fn the_meter_memo_is_invisible_and_never_inherited() {
    let c = swaying(2.0, 3);
    let before = format!("{c:?}");
    let m = c.speed_lower_bound().get();
    assert_eq!(format!("{c:?}"), before, "reading the meter changed Debug");
    assert_eq!(
        c.clone().speed_lower_bound().get().to_bits(),
        m.to_bits(),
        "clone"
    );
    let scaled = c.map_points(|p| Point3::new(p.x * 2.0, p.y * 2.0, p.z * 2.0));
    let ms = scaled.speed_lower_bound().get();
    assert!(
        (ms - 2.0 * m).abs() <= 1e-12 * m,
        "scaled net metered {ms}, twice the source's {m} expected"
    );
}

/// At `Sym` the memo is invisible to a session too: a curve whose meter
/// was first read before the session decides over its meter with the same
/// freezes as one whose meter is first read inside it. A memo served
/// across that boundary hands the session a node it never recorded,
/// which freezes to an unknown of its own.
#[test]
fn a_meter_read_before_a_session_is_not_served_inside_it() {
    use geom_core::k_stats::decide;
    use geom_core::predicate::{Band, Margin};
    use geom_core::sym::with_session;
    use geom_core::{Sym, SymBudget, Tol};
    type S = Sym<f64>;
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let in_session = |c: &NurbsCurve3<S>| {
        with_session(budget, || {
            let m = c.speed_lower_bound().get();
            let band = Band::linear(Tol::witness()).expect("linear band");
            decide("meter_memo", Margin::of(m - m), band)
        })
    };
    let read_before = swaying(2.0, 3).map_scalar(S::from_f64);
    let _ = read_before.speed_lower_bound();
    let fresh = swaying(2.0, 3).map_scalar(S::from_f64);
    // `m − m` cancels by id whichever node `m` is, so the decision is the
    // same either way; what a served pre-session meter changes is how
    // much of it the session could expand, which `frozen` counts.
    let (_, before_counts) = in_session(&read_before);
    let (_, inside_counts) = in_session(&fresh);
    assert_eq!(
        before_counts.frozen, inside_counts.frozen,
        "a meter first read before the session froze differently: {before_counts:?} against \
         {inside_counts:?}"
    );
}
