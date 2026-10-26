//! Review probes for PR 5724 (the turn rule's follow-ups). Not a gate:
//! `#[ignore]`d, run with
//! `cargo nextest run -p geom --test all review_turn_follow_ups_probes --run-ignored all --no-capture`.
//!
//! Every "truth" here is read at the TRUE defect, `[t0 + 2π, t1]` or
//! `[t1, t0 + 2π]` with `t0`, `t1` the f64 values and 2π the real number
//! (`Interval::tau()`), through the addition formula about the exact `t0`
//! so a far `t0` keeps a tight enclosure.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::carrier_speed::{turn_defect_speed_hi, turn_defect_speed_lo};
use geom_core::period::{Bounded, Extent, Period, Turn, TurnRows};
use geom_core::{Band, Bounds, Interval, Point3, Real, SpanLocate, Vec3};

const TAU: f64 = core::f64::consts::TAU;

fn ellipse(a: f64, b: f64) -> Curve3<f64> {
    Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        semi_u: a,
        semi_v: b,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn spiric(major: f64, minor: f64, offset: f64) -> Curve3<f64> {
    Curve3::Spiric {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        major_radius: major,
        minor_radius: minor,
        offset,
    }
}

/// `(sin, cos)` of `t0 + d`, `t0` an exact f64, `d` a small interval.
fn sc_at(t0: f64, d: Interval) -> (Interval, Interval) {
    let (s0, c0) = Interval::from_f64(t0).sin_cos();
    let (sd, cd) = d.sin_cos();
    (s0 * cd + c0 * sd, c0 * cd - s0 * sd)
}

fn ellipse_speed(a: f64, b: f64, t0: f64, d: Interval) -> Interval {
    let (s, c) = sc_at(t0, d);
    ((Interval::from_f64(a) * s).powi(2) + (Interval::from_f64(b) * c).powi(2)).sqrt()
}

fn spiric_speed(major: f64, minor: f64, offset: f64, t0: f64, d: Interval) -> Interval {
    let (s, c) = sc_at(t0, d);
    let (major, minor, offset) = (
        Interval::from_f64(major),
        Interval::from_f64(minor),
        Interval::from_f64(offset),
    );
    let f = ((major + minor * c).powi(2) - offset.powi(2)).sqrt();
    minor * (Interval::one() + (offset * s / f).powi(2)).sqrt()
}

/// The true defect `D = (t1 − t0) − 2π` (signed, an enclosure): the defect
/// is `t0 + hull(0, D)` modulo 2π.
fn true_defect(t0: f64, t1: f64) -> Interval {
    // `t1 − t0 = s + e` exactly (two-sum), `2π = τ + τ_lo` to ~1e-32.
    let s = t1 - t0;
    let bb = s - t1;
    let e = (t1 - (s - bb)) + (-t0 - bb);
    let tau_lo = 2.449_293_598_294_706_4e-16;
    let head = s - TAU;
    if (s - TAU).abs() > 1e-3 {
        return Interval::from_f64(t1) - Interval::from_f64(t0) - Interval::tau();
    }
    Interval::from_f64(head) + Interval::from_f64(e)
        - Interval::from_bounds(tau_lo - 1e-31, tau_lo + 1e-31)
}

struct Truth {
    /// An upper bound on the least speed over the defect.
    min_hi: f64,
    /// A lower bound on the least speed over the defect.
    min_lo: f64,
    /// A lower bound on the greatest speed over the defect.
    max_lo: f64,
    /// Enclosure of the defect's arc length.
    len: (f64, f64),
}

fn truth(speed: impl Fn(Interval) -> Interval, d: Interval) -> Truth {
    let n = 16;
    let (mut min_hi, mut min_lo, mut max_lo) = (f64::INFINITY, f64::INFINITY, 0.0_f64);
    let (mut len_lo, mut len_hi) = (0.0, 0.0);
    let piece_w = d.abs() / Interval::from_f64(f64::from(n));
    for i in 0..=n {
        let at = d * Interval::from_f64(f64::from(i) / f64::from(n));
        let s = speed(at);
        min_hi = min_hi.min(s.hi());
        max_lo = max_lo.max(s.lo());
        if i < n {
            let next = d * Interval::from_f64(f64::from(i + 1) / f64::from(n));
            let sp = speed(at.enclosure_hull(next));
            min_lo = min_lo.min(sp.lo());
            len_lo += piece_w.lo() * sp.lo().max(0.0);
            len_hi += piece_w.hi() * sp.hi();
        }
    }
    Truth {
        min_hi,
        min_lo,
        max_lo,
        len: (len_lo * (1.0 - 1e-15), len_hi * (1.0 + 1e-15)),
    }
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    lo_violations: Vec<String>,
    lo_iv_violations: Vec<String>,
    hi_underreads: Vec<String>,
    hi_iv_underreads: Vec<String>,
    /// Least `(min_lo − lo) / allowance` seen: < 1 means the allowance
    /// was needed; < 0 means the bound was saved only by sampling slack.
    least_slack_units: (f64, String),
    verdicts: std::collections::BTreeMap<String, usize>,
    bad_verdicts: Vec<String>,
    /// Least true length / 10ε over Over/Short verdicts.
    least_decided_ratio: (f64, String),
    /// Greatest true length / ε over Whole verdicts.
    most_whole_ratio: (f64, String),
    sane_least_decided: (f64, String),
    sane_most_whole: (f64, String),
}

fn verdict_name<E>(g: &Result<Turn, E>) -> &'static str {
    match g {
        Ok(Turn::Over(_)) => "over",
        Ok(Turn::Short(_)) => "short",
        Ok(Turn::Whole(_)) => "whole",
        Err(_) => "escalated",
    }
}

#[allow(clippy::too_many_arguments)]
fn check_verdict<E>(
    tally: &mut Tally,
    lane: &str,
    got: &Result<Turn, E>,
    d: Interval,
    len: (f64, f64),
    eps: f64,
    case: &str,
) {
    let v = verdict_name(got);
    *tally.verdicts.entry(format!("{lane} {v}")).or_default() += 1;
    match v {
        "over" | "short" => {
            let sign_ok = if v == "over" {
                d.lo() > 0.0
            } else {
                d.hi() < 0.0
            };
            if !sign_ok || len.0 <= eps {
                tally
                    .bad_verdicts
                    .push(format!("{lane} {v} but D {d:?} len {len:?}: {case}"));
            }
            let r = len.0 / (10.0 * eps);
            if case.contains("sub-ulp false") && r < tally.sane_least_decided.0 {
                tally.sane_least_decided = (r, format!("{lane} {v} {case} len {len:?}"));
            }
            if r < tally.least_decided_ratio.0 {
                tally.least_decided_ratio = (r, format!("{lane} {v} {case} len {len:?}"));
            }
        }
        "whole" => {
            let r = len.1 / eps;
            if case.contains("sub-ulp false") && r > tally.sane_most_whole.0 {
                tally.sane_most_whole = (r, format!("{lane} {case} len {len:?}"));
            }
            if r > tally.most_whole_ratio.0 {
                tally.most_whole_ratio = (r, format!("{lane} {case} len {len:?}"));
            }
            if len.0 > eps * (1.0 + 1e-9) {
                tally
                    .bad_verdicts
                    .push(format!("{lane} whole but len {len:?} > eps {eps}: {case}"));
            }
        }
        _ => {}
    }
}

/// Claims 1, 3 and 6 on conics.
#[test]
#[ignore = "review probe"]
fn conic_defect_bounds_and_verdicts_against_the_true_defect() {
    let rows = TurnRows::bounded("probe_turn", "probe_straddle");
    let mut tally = Tally {
        least_slack_units: (f64::INFINITY, String::new()),
        least_decided_ratio: (f64::INFINITY, String::new()),
        sane_least_decided: (f64::INFINITY, String::new()),
        ..Tally::default()
    };
    let big_k = (1e6 / core::f64::consts::PI).round();
    let shifts = [
        0.0,
        big_k * core::f64::consts::PI,
        -big_k * core::f64::consts::PI,
        1e6,
        -1e6,
        2048.0 * core::f64::consts::PI,
    ];
    for (a0, b0) in [
        (1.0, 1.0),
        (16.0, 1.0),
        (100.0, 1.0),
        (1e4, 1.0),
        (1e6, 1.0),
        (1.0, 1e4),
        (3e-3, 1e-7),
    ] {
        for (su, sv) in [(1.0, 1.0), (-1.0, -1.0)] {
            let (a, b) = (a0 * su, b0 * sv);
            let (hi_ax, lo_ax) = (a0.max(b0), a0.min(b0));
            // Centres: the vertices, the flank where |s′| ≈ the ceiling
            // (a few b/a off the slow vertex), and generic angles.
            let slow = if a0 >= b0 {
                0.0
            } else {
                core::f64::consts::FRAC_PI_2
            };
            let q = lo_ax / hi_ax;
            let mut centres = vec![0.0, core::f64::consts::FRAC_PI_2, 0.3, 2.5, -1.2];
            for k in [0.5, 1.0, 2.0, 5.0, 20.0, 100.0, 1e3] {
                centres.push(slow + k * q);
                centres.push(slow - k * q);
            }
            for shift in shifts {
                for &c in &centres {
                    let centre = shift + c;
                    let s_c = ellipse_speed(a, b, centre, Interval::zero()).lo();
                    let mut widths: Vec<f64> = (0..=30)
                        .map(|i| 10f64.powf(-16.0 + 0.5 * f64::from(i)))
                        .collect();
                    for eps in [1e-6, 1e-9, 1e-12] {
                        for k in [
                            0.5, 0.9, 0.99, 1.0, 1.01, 1.1, 9.0, 9.9, 9.99, 10.0, 10.01, 10.1,
                            11.0, 20.0,
                        ] {
                            widths.push(k * eps / s_c);
                            widths.push(k * eps / lo_ax);
                        }
                    }
                    for w in widths {
                        for sign in [1.0, -1.0] {
                            let h = sign * w; // headroom τ − extent, intended
                            let t0 = centre + 0.5 * h;
                            let t1 = t0 + (TAU - h);
                            if t1 <= t0 || w > 1.0 {
                                continue;
                            }
                            let extent = Extent::hull([t0, t1]).unwrap();
                            let hf = TAU - extent.get();
                            let m = t0 - 0.5 * hf;
                            let lo = turn_defect_speed_lo(&ellipse(a, b), t0, extent).get();
                            let hi = turn_defect_speed_hi(&ellipse(a, b), t0, extent).get();
                            let civ = ellipse(a, b).map_scalar(Interval::from_f64);
                            let ext_iv =
                                Extent::hull([Interval::from_f64(t0), Interval::from_f64(t1)])
                                    .unwrap();
                            let lo_iv_v =
                                turn_defect_speed_lo(&civ, Interval::from_f64(t0), ext_iv);
                            let hi_iv_v =
                                turn_defect_speed_hi(&civ, Interval::from_f64(t0), ext_iv);
                            let (lo_iv, hi_iv) = (lo_iv_v.get().lo(), hi_iv_v.get().hi());
                            let d = true_defect(t0, t1);
                            let tr = truth(|x| ellipse_speed(a, b, t0, x), d);
                            tally.cases += 1;
                            let case = format!("a {a} b {b} t0 {t0:e} t1 {t1:e} hf {hf:e} m {m:e}");
                            if lo > tr.min_hi {
                                tally
                                    .lo_violations
                                    .push(format!("lo {lo} > min_hi {}: {case}", tr.min_hi));
                            }
                            if lo_iv > tr.min_hi {
                                tally
                                    .lo_iv_violations
                                    .push(format!("lo_iv {lo_iv} > min_hi {}: {case}", tr.min_hi));
                            }
                            if hi < tr.max_lo {
                                tally.hi_underreads.push(format!(
                                    "hi {hi} < max_lo {} (rel {:e}): {case}",
                                    tr.max_lo,
                                    (tr.max_lo - hi) / hi
                                ));
                            }
                            if hi_iv < tr.max_lo {
                                tally
                                    .hi_iv_underreads
                                    .push(format!("hi_iv {hi_iv} < max_lo {}: {case}", tr.max_lo));
                            }
                            let allowance = hi_ax * (1.0 + m.abs()) * (1.0 / (1u64 << 46) as f64);
                            if lo > lo_ax {
                                let units = (tr.min_lo - lo) / allowance;
                                if units < tally.least_slack_units.0 {
                                    tally.least_slack_units = (units, case.clone());
                                }
                            }
                            for eps in [1e-6, 1e-9, 1e-12] {
                                let band = Band::new(eps, 10.0 * eps).unwrap();
                                let got = Period::new(TAU).turn(
                                    rows,
                                    extent,
                                    Bounded::new(
                                        geom_core::InfSpeed::new(lo),
                                        geom_core::SupSpeed::new(hi),
                                    ),
                                    band,
                                );
                                let ce =
                                    format!("eps {eps} sub-ulp {} {case}", hi_ax * 2.5e-16 > eps);
                                check_verdict(&mut tally, "f64", &got, d, tr.len, eps, &ce);
                                let got_iv = Period::new(Interval::tau()).turn(
                                    rows,
                                    ext_iv,
                                    Bounded::new(lo_iv_v, hi_iv_v),
                                    band,
                                );
                                check_verdict(&mut tally, "iv", &got_iv, d, tr.len, eps, &ce);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut cats = std::collections::BTreeMap::<String, (usize, f64, String)>::new();
    for b in &tally.bad_verdicts {
        let key: String = b.split(" but").next().unwrap().to_string()
            + if b.contains("sub-ulp true") {
                " sub-ulp"
            } else {
                " sane"
            };
        // worst ratio: len.lo / eps
        let lenlo: f64 = b
            .split("len (")
            .nth(1)
            .and_then(|r| r.split(',').next())
            .and_then(|x| x.parse().ok())
            .unwrap_or(0.0);
        let eps: f64 = b
            .split("eps ")
            .nth(1)
            .and_then(|r| r.split(' ').next())
            .and_then(|x| x.parse().ok())
            .unwrap_or(1.0);
        let e = cats.entry(key).or_insert((0, 0.0, String::new()));
        e.0 += 1;
        if lenlo / eps > e.1 {
            e.1 = lenlo / eps;
            e.2 = b.clone();
        }
    }
    for (k, v) in &cats {
        eprintln!("BAD {k}: {} worst len/eps {:.4} :: {}", v.0, v.1, v.2);
    }
    let sane: Vec<_> = tally
        .bad_verdicts
        .iter()
        .filter(|s| s.contains("sub-ulp false") && !s.contains("over") && !s.contains("short"))
        .collect();
    eprintln!("bad verdicts outside the sub-ulp regime: {}", sane.len());
    for s in sane.iter().take(12) {
        eprintln!("  {s}");
    }
    eprintln!("cases {}", tally.cases);
    eprintln!("verdicts {:?}", tally.verdicts);
    eprintln!(
        "least slack in allowance units {:?}",
        tally.least_slack_units
    );
    eprintln!("least decided len/10eps {:?}", tally.least_decided_ratio);
    eprintln!("most whole len/eps {:?}", tally.most_whole_ratio);
    eprintln!(
        "sane least decided len/10eps {:?}",
        tally.sane_least_decided
    );
    eprintln!("sane most whole len/eps {:?}", tally.sane_most_whole);
    for (name, v) in [
        ("lo violations", &tally.lo_violations),
        ("lo_iv violations", &tally.lo_iv_violations),
        ("hi underreads (f64)", &tally.hi_underreads),
        ("hi_iv underreads", &tally.hi_iv_underreads),
        ("bad verdicts", &tally.bad_verdicts),
    ] {
        eprintln!("{name}: {}", v.len());
        for s in v.iter().take(8) {
            eprintln!("  {s}");
        }
    }
}

/// Claim 2: the spiric sup over the defect, near tangency and on the pinch.
#[test]
#[ignore = "review probe"]
fn spiric_defect_sup_against_the_true_defect() {
    let pi = core::f64::consts::PI;
    let (mut bad_iv, mut bad_f64, mut n, mut nan) = (Vec::new(), Vec::new(), 0usize, 0usize);
    let mut worst_f64 = (0.0_f64, String::new());
    for (major, minor, offset) in [
        (1.0, 0.5, 0.0),
        (1.0, 0.5, 0.25),
        (1.0, 0.5, 0.5 - 1e-6),
        (1.0, 0.5, 0.5 - 1e-12),
        (1.0, 0.5, 0.5 - 1e-15),
        (1.0, 0.5, -(0.5 - 1e-15)),
        (1.0, 0.5, 0.5),
        (3.0, 1.0, -1.5),
        (3.0, 1.0, 2.0 - 1e-13),
        (100.0, 1e-3, 99.999 - 1e-12),
    ] {
        for shift in [0.0, 2.0 * pi * 1e5, -2.0 * pi * 3e5] {
            for c in [
                0.0,
                1.0,
                2.5,
                pi - 0.1,
                pi - 1e-4,
                pi - 1e-8,
                pi,
                pi + 1e-8,
                pi + 1e-4,
                4.0,
            ] {
                let centre = shift + c;
                for w in [0.0, 1e-15, 1e-12, 1e-9, 1e-6, 1e-3, 0.1, 1.0, 3.0] {
                    for sign in [1.0, -1.0] {
                        let h = sign * w;
                        let t0 = centre + 0.5 * h;
                        let t1 = t0 + (TAU - h);
                        let extent = Extent::hull([t0, t1]).unwrap();
                        let curve = spiric(major, minor, offset);
                        let hi = turn_defect_speed_hi(&curve, t0, extent).get();
                        let civ = curve.map_scalar(Interval::from_f64);
                        let ext_iv =
                            Extent::hull([Interval::from_f64(t0), Interval::from_f64(t1)]).unwrap();
                        let hi_iv =
                            turn_defect_speed_hi(&civ, Interval::from_f64(t0), ext_iv).get();
                        let d = true_defect(t0, t1);
                        n += 1;
                        let case = format!(
                            "R {major} r {minor} d {offset} centre {centre} w {h:e}: hi {hi} hi_iv {hi_iv:?}"
                        );
                        if !hi.is_finite() || !hi_iv.is_certified() {
                            nan += 1;
                        }
                        let n_s = 48;
                        for i in 0..=n_s {
                            let at = d * Interval::from_f64(f64::from(i) / f64::from(n_s));
                            let s = spiric_speed(major, minor, offset, t0, at);
                            if !s.is_certified() {
                                continue;
                            }
                            if hi_iv.is_certified() && hi_iv.hi() < s.lo() {
                                bad_iv.push(format!("{case}: speed {s:?} at {i}"));
                            }
                            if hi.is_finite() && hi < s.lo() {
                                let r = (s.lo() - hi) / hi;
                                if r > worst_f64.0 {
                                    worst_f64 = (r, case.clone());
                                }
                                bad_f64.push(format!("{case}: speed {s:?} rel {r:e}"));
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("spiric cases {n}, non-finite sups {nan}");
    eprintln!("iv under-reads {}", bad_iv.len());
    for s in bad_iv.iter().take(8) {
        eprintln!("  {s}");
    }
    eprintln!("f64 under-reads {} worst {:?}", bad_f64.len(), worst_f64);
    for s in bad_f64.iter().take(8) {
        eprintln!("  {s}");
    }
}
