//! **The integral speed meter answers on a curve that sways across its
//! chords, and stays a lower bound.** A corner path that swings side to
//! side has derivative coefficients pointing back along every control
//! chord, so the global and per-span chord assemblies go negative on a
//! curve that never stalls; the piece assembly reads the derivative's
//! Bernstein coefficients on sixteen pieces per span, which converge on
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
        let g = 0.5 * (5.0_f64.sqrt() - 1.0);
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
/// (`m / ceiling` from 0.99 at `a = 0.75` to 0.57 at degree 3,
/// `a = 2`): a schedule change that gives most of it back is visible
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
