//! R1 review probes for PR #369 (M8-14 long-turn integral meter).
//!
//! Independent adversarial verification, written blind to the PR's own
//! suite: (1) join-bound soundness against densely sampled + locally
//! refined true minimum speed on adversarial integral carriers;
//! (2) never-weaker vs a local replica of the retired global-chord arm;
//! (3) abstention/poison logic of the join (one-sided collapse, both
//! collapse, poisoned input laundering).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss
)]
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use test_utils::fuzz;

/// The retired global-chord arm, replicated independently.
fn old_arm(c: &NurbsCurve3<f64>) -> f64 {
    let p = c.degree();
    if p == 0 || c.control().len() < 2 {
        return f64::NAN;
    }
    let knots = c.knots().knots();
    let ctrl = c.control();
    let chord = *ctrl.last().unwrap() - *ctrl.first().unwrap();
    let d = chord / chord.norm();
    let mut acc = f64::INFINITY;
    for i in 0..(ctrl.len() - 1) {
        let scale = (p as f64) / (knots[i + p + 1] - knots[i + 1]);
        acc = acc.min(d.dot((ctrl[i + 1] - ctrl[i]) * scale));
    }
    acc
}

/// Densely sampled minimum speed with local ternary refinement around
/// the coarse argmin — an upper bound on the true infimum that is
/// tight enough to catch any real excess.
///
/// The oracle's sample count rides `CAD_FUZZ_EFFORT` (callers pass
/// `fuzz::scaled`); the 200-step ternary polish afterwards does not — it
/// is a convergence budget, not a sweep, and 200 thirds is already far
/// past f64 resolution.
fn sampled_min_speed(c: &NurbsCurve3<f64>, n: usize) -> f64 {
    let (lo, hi) = c.domain();
    let mut smin = f64::INFINITY;
    let mut tmin = lo;
    for i in 0..=n {
        let t = lo + (hi - lo) * (i as f64) / (n as f64);
        let s = c.deriv(t).norm();
        if s < smin {
            smin = s;
            tmin = t;
        }
    }
    // Ternary refinement in a one-step bracket around the argmin.
    let h = (hi - lo) / (n as f64);
    let (mut a, mut b) = ((tmin - h).max(lo), (tmin + h).min(hi));
    for _ in 0..200 {
        let m1 = a + (b - a) / 3.0;
        let m2 = b - (b - a) / 3.0;
        if c.deriv(m1).norm() < c.deriv(m2).norm() {
            b = m2;
        } else {
            a = m1;
        }
    }
    smin.min(c.deriv(0.5 * (a + b)).norm())
}

/// Soundness + never-weaker on one carrier: if the join is real it
/// must not exceed the refined sampled min speed, and it must not be
/// below the old arm where the old arm was real.
fn check(name: &str, c: &NurbsCurve3<f64>) {
    let m = c.speed_lower_bound();
    let old = old_arm(c);
    if !m.is_nan() {
        let truth = sampled_min_speed(c, fuzz::scaled(2_000));
        assert!(
            m <= truth + truth.abs() * 1e-9 + 1e-12,
            "{name}: UNSOUND — join bound {m} exceeds refined sampled min speed \
             {truth} — {}",
            fuzz::replay()
        );
        if !old.is_nan() {
            // The never-weaker gate: this file's unique contribution, and
            // O(n) in the control net — never trimmed.
            assert!(
                m >= old,
                "{name}: WEAKENED — join {m} below the old global-chord arm \
                 {old} — {}",
                fuzz::replay()
            );
        }
    } else {
        // Poison join with a real old arm would be a capability
        // regression on a structurally valid carrier.
        assert!(
            old.is_nan() || !(old > 0.0),
            "{name}: join poisoned where the old arm was green ({old}) — {}",
            fuzz::replay()
        );
    }
}

fn interp(pts: &[Point3<f64>], deg: usize) -> NurbsCurve3<f64> {
    NurbsCurve3::<f64>::interpolate(pts, deg).unwrap()
}

fn helix(turns: f64, scale: f64) -> NurbsCurve3<f64> {
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = (k as f64) / (n as f64);
            let a = std::f64::consts::TAU * turns * s;
            Point3::new(scale * a.cos(), scale * a.sin(), scale * 0.4 * turns * s)
        })
        .collect();
    interp(&pts, 3)
}

/// Adversarial soundness sweep: long turns, near-cusps, inflected
/// S-curves, tiny/huge scales, high degree, dense interpolations.
#[test]
fn r1_adversarial_soundness() {
    for turns in [0.5, 0.75, 1.0, 1.5, 2.0, 3.0] {
        check(&format!("helix_{turns}"), &helix(turns, 1.0));
    }
    check("helix_tiny", &helix(1.5, 1e-6));
    check("helix_huge", &helix(1.5, 1e6));
    // Near-cusp: t^2 + eps*t advance — speed min small but positive.
    for eps in [1e-1, 1e-3, 1e-6] {
        let pts: Vec<Point3<f64>> = (0..=40)
            .map(|i| {
                let t = f64::from(i) / 40.0 - 0.5;
                Point3::new(t * t + eps * t, 0.3 * t, 0.0)
            })
            .collect();
        check(&format!("near_cusp_{eps}"), &interp(&pts, 3));
    }
    // S-curve with inflections at several frequencies and degrees.
    for deg in [2, 3, 5, 7] {
        let pts: Vec<Point3<f64>> = (0..=60)
            .map(|i| {
                let t = f64::from(i) / 60.0;
                Point3::new(t, 0.4 * (t * 12.0).sin(), 0.2 * (t * 5.0).cos())
            })
            .collect();
        check(&format!("s_curve_deg{deg}"), &interp(&pts, deg));
    }
    // Dense interpolation (many knots) on a slow spiral.
    let pts: Vec<Point3<f64>> = (0..=300)
        .map(|i| {
            let t = f64::from(i) / 300.0;
            let a = 4.0 * std::f64::consts::PI * t;
            Point3::new((1.0 + t) * a.cos(), (1.0 + t) * a.sin(), t)
        })
        .collect();
    check("dense_spiral", &interp(&pts, 3));
    // Exact near-antipodal planar arc (the old arm's killer).
    #[allow(clippy::approx_constant)]
    let near_antipodal = [3.141_592_6, std::f64::consts::PI, 3.15];
    for curl in near_antipodal {
        let pts: Vec<Point3<f64>> = (0..=32)
            .map(|k| {
                let a = curl * f64::from(k) / 32.0;
                Point3::new(a.sin(), 1.0 - a.cos(), 0.0)
            })
            .collect();
        check(&format!("arc_{curl}"), &interp(&pts, 3));
    }
}

/// Randomized direct-construction soundness: random control nets,
/// random degrees, interior knot multiplicities, mixed scales. Any
/// real join bound must sit at or below the sampled true min speed.
#[test]
fn r1_randomized_soundness() {
    let mut rng = fuzz::start("lt_r1_probes::randomized_soundness");
    for case in 0..fuzz::scaled(50) {
        let p = 2 + (case % 4); // degree 2..=5
        let extra = case % 13;
        let n_ctrl = p + 1 + extra;
        let scale = [1.0, 1e-6, 1e6][case % 3];
        // Clamped knots; every third case gets a doubled interior knot.
        let interior = n_ctrl - p - 1;
        let mut knots = vec![0.0; p + 1];
        let mut vals: Vec<f64> = (1..=interior)
            .map(|i| i as f64 / (interior as f64 + 1.0))
            .collect();
        if case % 3 == 0 && vals.len() >= 2 {
            let v = vals[0];
            vals[1] = v;
        }
        knots.extend(vals);
        knots.extend(vec![1.0; p + 1]);
        let Ok(kv) = KnotVector::clamped(knots, p) else {
            continue;
        };
        let ctrl: Vec<Point3<f64>> = (0..n_ctrl)
            .map(|_| Point3::new(scale * rng.unit(), scale * rng.unit(), scale * rng.unit()))
            .collect();
        let Ok(c) = NurbsCurve3::<f64>::new(kv, ctrl, vec![1.0; n_ctrl]) else {
            continue;
        };
        check(&format!("random_{case}"), &c);
    }
}

/// Full-precision old-arm values on the PR's green corpus rows, to
/// diff bit-for-bit against the shipped meter at the merge base
/// (`--nocapture`).
#[test]
fn r1_head_old_arm_full_precision() {
    let interp3 = |pts: &[Point3<f64>]| NurbsCurve3::<f64>::interpolate(pts, 3).unwrap();
    let gentle: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0;
            Point3::new(t, 0.15 * (t * 6.0).sin(), 0.05 * t * t)
        })
        .collect();
    let split: Vec<Point3<f64>> = (0..=30)
        .map(|i| {
            let t = f64::from(i) / 30.0;
            Point3::new(t, 0.1 * (t * 4.0).sin(), 0.02 * t)
        })
        .collect();
    let (a, b, c) = (2.2, 1.3, 1.5);
    let twisted: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let t = 2.0f64.mul_add(f64::from(k) / 32.0, -1.0);
            Point3::new(a * t, b * t * t, c * t * t * t)
        })
        .collect();
    let mut rows: Vec<(String, NurbsCurve3<f64>)> = vec![
        ("gentle_cubic".into(), interp3(&gentle)),
        ("split_carrier".into(), interp3(&split)),
        ("twisted_cubic".into(), interp3(&twisted)),
    ];
    for curl in [0.45, 1.0, 2.0, 2.5, 3.0, 4.7] {
        let pts: Vec<Point3<f64>> = (0..=32)
            .map(|k| {
                let a = curl * f64::from(k) / 32.0;
                Point3::new(a.sin(), 1.0 - a.cos(), 0.0)
            })
            .collect();
        rows.push((format!("arc_{curl}"), interp3(&pts)));
    }
    for turns in [0.5, 1.0, 2.0, 3.0] {
        rows.push((format!("helix_{turns}"), helix(turns, 1.0)));
    }
    for (name, curve) in &rows {
        println!("HEAD {name} {:?}", old_arm(curve));
    }
}

/// Minimum slack (truth − bound) across the sound rows, printed —
/// the review's "worst bound-vs-truth margin" number (`--nocapture`).
#[test]
fn r1_min_slack_report() {
    let mut worst: (String, f64) = (String::new(), f64::INFINITY);
    let mut rows: Vec<(String, NurbsCurve3<f64>)> = vec![];
    for turns in [0.5, 1.0, 2.0, 3.0] {
        rows.push((format!("helix_{turns}"), helix(turns, 1.0)));
    }
    for eps in [1e-1, 1e-3, 1e-6] {
        let pts: Vec<Point3<f64>> = (0..=40)
            .map(|i| {
                let t = f64::from(i) / 40.0 - 0.5;
                Point3::new(t * t + eps * t, 0.3 * t, 0.0)
            })
            .collect();
        rows.push((format!("near_cusp_{eps}"), interp(&pts, 3)));
    }
    for curl in [3.0, std::f64::consts::PI, 6.0] {
        let pts: Vec<Point3<f64>> = (0..=32)
            .map(|k| {
                let a = curl * f64::from(k) / 32.0;
                Point3::new(a.sin(), 1.0 - a.cos(), 0.0)
            })
            .collect();
        rows.push((format!("arc_{curl}"), interp(&pts, 3)));
    }
    for (name, curve) in &rows {
        let m = curve.speed_lower_bound();
        if m.is_nan() || !(m > 0.0) {
            println!("SLACK {name} bound {m:?} (not positive; skipped)");
            continue;
        }
        let truth = sampled_min_speed(curve, fuzz::scaled(2_000));
        let rel = (truth - m) / truth;
        println!("SLACK {name} bound {m:?} truth {truth:?} rel_slack {rel:.3e}");
        if rel < worst.1 {
            worst = (name.clone(), rel);
        }
    }
    println!("WORST-SLACK {} {:.3e}", worst.0, worst.1);
}

/// The join's abstention logic, exercised structurally.
#[test]
fn r1_join_abstention_logic() {
    // (1) Global chord exactly collapsed (first == last control point),
    // per-span chords all nonzero: the global arm abstains and the
    // per-span arm answers alone — real, sound, positive here.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let square = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.5, 0.0),
            Point3::new(0.5, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0), // == first: global chord 0/0
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let m = square.speed_lower_bound();
    assert!(
        !m.is_nan(),
        "global-chord collapse must ABSTAIN, not poison the join (got NaN)"
    );
    let truth = sampled_min_speed(&square, fuzz::scaled(2_000));
    assert!(m <= truth + 1e-12, "abstention case unsound: {m} > {truth}");

    // (2) A per-span chord exactly collapsed (P_span == P_{span-p} for
    // one span) while the global chord is fine: the per-span arm
    // abstains and the join must equal the global arm's real value
    // exactly — never NaN, never a fabricated max.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0), // == P_0: span 2's chord is 0/0
            Point3::new(2.0, 0.5, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    let old = old_arm(&c);
    assert!(
        !m.is_nan() && (m - old).abs() == 0.0,
        "per-span collapse must abstain and hand back the global arm \
         bit-for-bit: got {m}, global {old}"
    );

    // (3) BOTH collapsed: closed square whose middle span chord also
    // collapses — the join must be poison, not a fabricated bound.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 0.7, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0), // span chord P_0..P_2 collapses? no: P_2 == P_0 collapses span 2
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0), // == first: global collapses
        ],
        vec![1.0; 5],
    )
    .unwrap();
    // Global chord: last == first -> abstains. Span 2 chord P_0..P_2
    // collapses -> per-span poisons. Both abstain -> poison.
    let m = c.speed_lower_bound();
    assert!(
        m.is_nan(),
        "both assemblies collapsed: the join must be poison, got {m}"
    );

    // (4) Poison INPUT (non-finite interior control point, finite
    // ends): both assemblies must transmit poison — no laundering.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(f64::NAN, 0.5, 0.0),
            Point3::new(2.0, 0.5, 0.0),
            Point3::new(3.0, 0.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap_or_else(|_| panic!("constructor rejects NaN — adjust probe"));
    let m = c.speed_lower_bound();
    assert!(
        m.is_nan(),
        "a poisoned control point must poison the join, got {m}"
    );
}
