//! M8-14 (#222): the **long-turn integral meter** — per-span chord
//! directions for the integral arm of `speed_lower_bound`.
//!
//! The integral arm used to project every derivative control point on
//! ONE global chord direction (first→last control point), so any
//! integral carrier whose tangent turns ≥ ~half a revolution away from
//! its own chord reported a non-positive meter and every consumer
//! (`nurbs_span_meter`, `split_edge`) escalated — the #222 frontier:
//! helical sweep paths ≥ 0.5 turn refused while their speed never
//! dropped. This suite is the unit's record:
//!
//! - a BEFORE/AFTER corpus of existing green carriers, pinned as
//!   floors — the extension may only tighten or hold, never weaken
//!   (the measured baselines are the old single-chord values);
//! - the new capability rows: half-turn, full-turn and multi-turn
//!   helix interpolants, and a closed planar loop, meter positively
//!   AND soundly (densely sampled true speed never dips below);
//! - the refusal that must SURVIVE: a genuine stationary point (a
//!   cusp — the control net doubles back inside one span) still
//!   reports a non-positive meter under EVERY direction choice, and
//!   illegal structure still poisons.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// `!(m > 0.0)` is deliberate: refusal rows accept EITHER a
// non-positive bound or poison, and `m <= 0.0` would let a NaN pass.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;

/// The speed-meter suite's gently curved cubic (its first row).
fn gentle_cubic() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0;
            Point3::new(t, 0.15 * (t * 6.0).sin(), 0.05 * t * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The split-meter suite's rung-3 carrier.
fn split_carrier() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=30)
        .map(|i| {
            let t = f64::from(i) / 30.0;
            Point3::new(t, 0.1 * (t * 4.0).sin(), 0.02 * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The skinned demo's twisted cubic spine (`At, Bt², Ct³`, 33 points).
fn twisted_cubic() -> NurbsCurve3<f64> {
    let (a, b, c) = (2.2, 1.3, 1.5);
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let t = 2.0f64.mul_add(f64::from(k) / 32.0, -1.0);
            Point3::new(a * t, b * t * t, c * t * t * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// A planar circular arc of total turn `curl` radians, unit radius,
/// interpolated at 33 exact points — the lily blade spine's shape
/// (PR #316's probe table swept exactly this family).
fn planar_arc(curl: f64) -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let a = curl * f64::from(k) / 32.0;
            Point3::new(a.sin(), 1.0 - a.cos(), 0.0)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// A helix arc of `turns` revolutions, radius 1, pitch 0.4 per turn,
/// interpolated at 33 exact points per revolution (min 33).
fn helix(turns: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = f64::from(u32::try_from(k).unwrap()) / f64::from(u32::try_from(n).unwrap());
            let a = std::f64::consts::TAU * turns * s;
            Point3::new(a.cos(), a.sin(), 0.4 * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The full closed circle from the speed-meter suite's doubling-back
/// row (its retirement flips it here).
fn closed_circle() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0;
            let a = std::f64::consts::TAU * t;
            Point3::new(a.cos(), a.sin(), 0.0)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// Meter + soundness: the bound is positive and the densely sampled
/// true speed never dips below it. Returns the meter.
fn assert_positive_and_sound(name: &str, c: &NurbsCurve3<f64>) -> f64 {
    let m = c.speed_lower_bound();
    assert!(m > 0.0, "{name}: expected a positive meter, got {m}");
    let (lo, hi) = c.domain();
    for i in 0..=4000 {
        let t = lo + (hi - lo) * f64::from(i) / 4000.0;
        let s = c.deriv(t).norm();
        assert!(
            s >= m - 1e-12,
            "{name}: meter {m} exceeds the real speed {s} at t = {t}"
        );
    }
    m
}

/// The OLD integral arm, verbatim (pre-#222 `speed_lower_bound`): min
/// over ALL derivative control points of the projection on the ONE
/// global chord direction (first→last control point). Kept here as an
/// independent reference so the no-weakening constraint is a LIVE
/// invariant — `speed_lower_bound` must never fall below this on a
/// carrier where this was already a (positive) bound — rather than a
/// table of recorded floats. Verified bit-identical to the shipped
/// meter at this unit's base commit (the pre-change corpus run).
fn old_global_chord_bound(c: &NurbsCurve3<f64>) -> f64 {
    let p = c.degree();
    if p == 0 || c.control().len() < 2 {
        return f64::NAN;
    }
    let knots = c.knots().knots();
    let control = c.control();
    let chord = *control.last().unwrap() - *control.first().unwrap();
    let d = chord / chord.norm();
    let mut acc = f64::INFINITY;
    for i in 0..(control.len() - 1) {
        #[allow(clippy::cast_precision_loss)]
        let scale = (p as f64) / (knots[i + p + 1] - knots[i + 1]);
        let q = (control[i + 1] - control[i]) * scale;
        acc = acc.min(d.dot(q));
    }
    acc
}

/// The corpus rows whose OLD meter was already green (positive).
fn green_corpus() -> Vec<(&'static str, NurbsCurve3<f64>)> {
    vec![
        ("gentle_cubic", gentle_cubic()),
        ("split_carrier", split_carrier()),
        ("twisted_cubic", twisted_cubic()),
        ("arc_curl_0.45", planar_arc(0.45)),
        ("arc_curl_1.0", planar_arc(1.0)),
        ("arc_curl_1.5", planar_arc(1.5)),
        ("arc_curl_2.0", planar_arc(2.0)),
        ("arc_curl_2.5", planar_arc(2.5)),
        ("arc_curl_2.8", planar_arc(2.8)),
        ("arc_curl_3.0", planar_arc(3.0)),
        ("helix_0.25", helix(0.25)),
        // Full INTEGER turns were already green under the old arm —
        // the global chord is the helix axis and the bound was the
        // bare pitch rate (0.4·turns m/param, measured at base:
        // 0.4/0.8/1.2 against true speeds 6.3/12.6/18.9). The wall
        // was the near-half-turn band, where the chord goes
        // near-antipodal to the tangent. These rows pin that the
        // per-span answer beats even the axis bound.
        ("helix_1.0", helix(1.0)),
        ("helix_2.0", helix(2.0)),
        ("helix_3.0", helix(3.0)),
    ]
}

/// Constraint (a): every previously green carrier meters at least its
/// old single-chord value, and stays sound.
#[test]
fn the_green_corpus_only_tightens_or_holds() {
    for (name, c) in green_corpus() {
        let old = old_global_chord_bound(&c);
        assert!(old > 0.0, "{name}: corpus rot — the OLD arm was green here");
        let m = assert_positive_and_sound(name, &c);
        assert!(
            m >= old,
            "{name}: the per-span meter {m} fell below the old single-chord \
             value {old} — the extension weakened a green case"
        );
    }
}

/// The capability this unit exists for: long-turn integral carriers
/// whose speed never collapses now meter positively and soundly —
/// half-turn, full-turn and multi-turn helix arcs, the long planar
/// arcs past PR #316's curl wall, and the full closed circle (the
/// speed-meter suite's doubling-back row, flipped per its own
/// retirement text: the contract is SPEED, never injectivity).
#[test]
fn long_turn_carriers_meter_positively() {
    assert_positive_and_sound("helix_0.5", &helix(0.5));
    assert_positive_and_sound("helix_1.0", &helix(1.0));
    assert_positive_and_sound("helix_2.0", &helix(2.0));
    assert_positive_and_sound("helix_3.0", &helix(3.0));
    assert_positive_and_sound("arc_curl_3.0", &planar_arc(3.0));
    assert_positive_and_sound("arc_curl_4.7", &planar_arc(4.7));
    assert_positive_and_sound("arc_curl_6.0", &planar_arc(6.0));
    assert_positive_and_sound("closed_circle", &closed_circle());
}

/// Constraint (b): a GENUINE stationary point still refuses. The
/// control net doubles back inside a single span, so `C′` crosses
/// zero — no unit direction, global or per-span, can give a positive
/// convex-hull minimum there, and the honest answer stays
/// non-positive (or poison). This is the integral analogue of the
/// rational arm's cusp row.
#[test]
fn a_genuine_cusp_still_refuses() {
    // Degree 2, one span, control net out-and-back: C′ hits zero at
    // the turn-around.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ],
        vec![1.0, 1.0, 1.0],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    assert!(
        !(m > 0.0),
        "a cusped carrier must refuse (non-positive or poison), got {m}"
    );

    // And a smooth interpolated turn-around (the same shape the old
    // suite's doubling-back row really guarded): speed collapses at
    // the apex, so the meter must not be positive.
    let pts: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0 - 0.5;
            Point3::new(t * t, 0.0, 0.0)
        })
        .collect();
    let c = NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap();
    let m = c.speed_lower_bound();
    assert!(
        !(m > 0.0),
        "a turn-around carrier (zero speed at the apex) must refuse, got {m}"
    );
}

/// The corpus table, printed for the PR record (`--nocapture`).
#[test]
fn meter_corpus_table() {
    let rows: Vec<(String, NurbsCurve3<f64>)> = vec![
        ("gentle_cubic".into(), gentle_cubic()),
        ("split_carrier".into(), split_carrier()),
        ("twisted_cubic".into(), twisted_cubic()),
        ("arc_curl_0.45".into(), planar_arc(0.45)),
        ("arc_curl_1.0".into(), planar_arc(1.0)),
        ("arc_curl_1.5".into(), planar_arc(1.5)),
        ("arc_curl_2.0".into(), planar_arc(2.0)),
        ("arc_curl_2.5".into(), planar_arc(2.5)),
        ("arc_curl_2.8".into(), planar_arc(2.8)),
        ("arc_curl_3.0".into(), planar_arc(3.0)),
        ("arc_curl_3.5".into(), planar_arc(3.5)),
        ("arc_curl_4.7".into(), planar_arc(4.7)),
        ("arc_curl_6.0".into(), planar_arc(6.0)),
        ("helix_0.25".into(), helix(0.25)),
        ("helix_0.5".into(), helix(0.5)),
        ("helix_0.75".into(), helix(0.75)),
        ("helix_1.0".into(), helix(1.0)),
        ("helix_2.0".into(), helix(2.0)),
        ("helix_3.0".into(), helix(3.0)),
        ("closed_circle".into(), closed_circle()),
    ];
    println!("carrier | old single-chord | speed_lower_bound | sampled min speed");
    for (name, c) in &rows {
        let old = old_global_chord_bound(c);
        let m = c.speed_lower_bound();
        let (lo, hi) = c.domain();
        let mut smin = f64::INFINITY;
        for i in 0..=4000 {
            let t = lo + (hi - lo) * f64::from(i) / 4000.0;
            let s = c.deriv(t).norm();
            if s < smin {
                smin = s;
            }
        }
        println!("{name} | {old:.6} | {m:.6} | {smin:.6}");
    }
}
