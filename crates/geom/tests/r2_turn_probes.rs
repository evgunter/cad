//! PR 5686 review R2 probes (not for merge): the turn rule against the
//! true arc length of an ellipse's turn defect, `turn_defect_speed_hi`
//! against the old `ConicArc` expression and against sampled speeds, and
//! the spiric's sup.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::carrier_speed::{speed_range, turn_defect_speed_hi};
use geom_core::period::{Bounded, Extent, Period, Turn, TurnRows};
use geom_core::{Band, InfSpeed, Point3, Vec3};

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

fn speed(a: f64, b: f64, t: f64) -> f64 {
    ((a * t.sin()).powi(2) + (b * t.cos()).powi(2)).sqrt()
}

/// Arc length over `[lo, hi]`, composite Simpson.
fn arc(a: f64, b: f64, lo: f64, hi: f64) -> f64 {
    let n = 256;
    let h = (hi - lo) / f64::from(n);
    let mut s = speed(a, b, lo) + speed(a, b, hi);
    for i in 1..n {
        let w = if i % 2 == 1 { 4.0 } else { 2.0 };
        s += w * speed(a, b, lo + h * f64::from(i));
    }
    s * h / 3.0
}

/// Claim 2, bits: the old `ConicArc` sup expression, verbatim.
fn old_sup<T: geom_core::Real>(a: T, b: T, t0: T, gap: T) -> T {
    let (sm, cm) = (t0 - gap * T::from_f64(0.5)).sin_cos();
    ((a * sm).powi(2) + (b * cm).powi(2)).sqrt()
        + a.abs().max(b.abs()) * gap.abs() * T::from_f64(0.5)
}

#[test]
fn r2_turn_defect_speed_hi_is_the_old_bits_and_an_upper_bound() {
    let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        (x >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut worst: f64 = 0.0;
    for _ in 0..200_000 {
        let a = (rnd() * 8.0 - 4.0).exp() * if rnd() < 0.2 { -1.0 } else { 1.0 };
        let b = (rnd() * 8.0 - 4.0).exp() * if rnd() < 0.1 { -1.0 } else { 1.0 };
        let t0 = rnd() * 40.0 - 20.0;
        let gap = (rnd() * 2.0 - 1.0) * (rnd() * 30.0 - 25.0).exp();
        let got = turn_defect_speed_hi(&ellipse(a, b), t0, gap).get();
        assert_eq!(
            got.to_bits(),
            old_sup(a, b, t0, gap).to_bits(),
            "a {a} b {b} t0 {t0} gap {gap}"
        );
        // Upper bound over the defect: middle t0 - gap/2, width |gap|.
        let m = t0 - 0.5 * gap;
        for i in 0..=64 {
            let t = m + gap.abs() * (f64::from(i) / 64.0 - 0.5);
            let s = speed(a, b, t);
            let rel = (s - got) / got;
            worst = worst.max(rel);
            assert!(
                s <= got * (1.0 + 4.0 * f64::EPSILON),
                "speed {s} > sup {got}: a {a} b {b} t0 {t0} gap {gap}"
            );
        }
    }
    eprintln!("r2: worst (sampled - sup)/sup = {worst:e}");
}

/// Claim 3: the rule's verdicts against the true metric length of the
/// defect, for eccentric ellipses on the fast, slow and in-between side.
#[test]
fn r2_the_turn_rule_is_sound_against_the_true_defect_length() {
    let rows = TurnRows::bounded("r2_turn", "r2_turn_straddle");
    let one = Period::new(TAU);
    let mut tally = std::collections::BTreeMap::<(String, &str), usize>::new();
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for (a, b) in [
            (1.0001_f64, 1.0_f64),
            (2.0, 1.0),
            (16.0, 1.0),
            (20.0, 1.0),
            (100.0, 1.0),
            (1e4, 1.0),
            (1.0, 20.0),
            (-20.0, 1.0),
            (0.05, 0.001),
        ] {
            let inf = a.abs().min(b.abs());
            for centre in [
                0.0,
                0.3,
                0.7,
                1.0,
                1.4,
                core::f64::consts::FRAC_PI_2,
                2.5,
                4.0,
            ] {
                let local = speed(a, b, centre);
                for (rate_name, rate) in [
                    ("inf", inf),
                    ("local", local),
                    ("max", a.abs().max(b.abs())),
                ] {
                    for k in [
                        0.0, 0.3, 0.5, 0.9, 0.99, 1.0, 1.01, 1.1, 2.0, 3.0, 9.0, 9.9, 10.0, 10.1,
                        11.0, 20.0, 100.0, 1e3,
                    ] {
                        for over in [true, false] {
                            let defect = if over {
                                k * eps / rate
                            } else {
                                -k * eps / rate
                            };
                            let t0 = centre - 0.5 * defect;
                            let t1 = t0 + TAU + defect;
                            let Ok(extent) =
                                Extent::forward("r2_fwd", t0, t1, InfSpeed::new(inf), band)
                            else {
                                panic!("forward");
                            };
                            let h = TAU - extent.get();
                            let sup = turn_defect_speed_hi(&ellipse(a, b), t0, h);
                            let got = one.turn(
                                rows,
                                extent,
                                Bounded {
                                    inf: InfSpeed::new(inf),
                                    sup,
                                },
                                band,
                            );
                            // The defect stretch, centred at t0 - h/2.
                            let mid = t0 - 0.5 * h;
                            let len = arc(a, b, mid - 0.5 * h.abs(), mid + 0.5 * h.abs());
                            let case = format!(
                                "eps {eps} a {a} b {b} centre {centre} {rate_name} k {k} over {over}: h {h:e} len {len:e} got {got:?}"
                            );
                            let slack = 1.0 + 1e-9;
                            let verdict = match got {
                                Ok(Turn::Over(_)) => {
                                    assert!(h < 0.0, "{case}");
                                    assert!(
                                        len * slack >= 10.0 * eps,
                                        "Over not past the band: {case}"
                                    );
                                    "over"
                                }
                                Ok(Turn::Short(_)) => {
                                    assert!(h > 0.0, "{case}");
                                    assert!(
                                        len * slack >= 10.0 * eps,
                                        "Short not past the band: {case}"
                                    );
                                    "short"
                                }
                                Ok(Turn::Whole(_)) => {
                                    assert!(
                                        len <= eps * slack,
                                        "Whole not within the band: {case}"
                                    );
                                    "whole"
                                }
                                Err(u) => {
                                    // Escalation soundness of over_at_sup:
                                    // where the sup is not definitely over,
                                    // any overlap is within the escalation band.
                                    if h < 0.0 && !u.over_at_sup {
                                        assert!(
                                            len <= 10.0 * eps * slack,
                                            "over_at_sup false but overlap past the band: {case}"
                                        );
                                    }
                                    if u.over_at_sup {
                                        assert!(h < 0.0, "{case}");
                                    }
                                    "escalated"
                                }
                            };
                            *tally
                                .entry((format!("eps {eps:e} a/b {}", a / b), verdict))
                                .or_default() += 1;
                        }
                    }
                }
            }
        }
    }
    for (k, v) in &tally {
        eprintln!("r2 tally {k:?}: {v}");
    }
}

/// Claim 3, spiric: the sup the certification spiric arm reads is finite
/// for a whole oval (|d| < R − r), an upper bound on the sampled speed,
/// and what the rule does with it at the tangent limit.
#[test]
fn r2_spiric_sup_is_finite_on_whole_ovals_and_bounds_the_speed() {
    let spiric = |major: f64, minor: f64, offset: f64| Curve3::Spiric {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        major_radius: major,
        minor_radius: minor,
        offset,
    };
    for (major, minor, offset) in [
        (1.0, 0.5, 0.0),
        (1.0, 0.5, 0.25),
        (1.0, 0.5, 0.49),
        (1.0, 0.5, 0.5 - 1e-6),
        (1.0, 0.5, 0.5 - 1e-12),
        (0.09375, 0.0703125, 0.0078125),
        (3.0, 1.0, -1.5),
    ] {
        let c = spiric(major, minor, offset);
        let (lo, hi) = speed_range(&c);
        let (lo, hi) = (lo.get(), hi.get());
        assert!(
            hi.is_finite(),
            "sup finite: R {major} r {minor} d {offset}: {hi}"
        );
        let sup2 = turn_defect_speed_hi(&c, 0.3, 1e-9).get();
        assert_eq!(
            sup2.to_bits(),
            hi.to_bits(),
            "spiric defect sup is the global sup"
        );
        let n = 20_000;
        let mut max: f64 = 0.0;
        for i in 0..n {
            let t = TAU * f64::from(i) / f64::from(n);
            let dt = 1e-7;
            let p = c.eval(t + dt);
            let q = c.eval(t - dt);
            let d = ((p.x - q.x).powi(2) + (p.y - q.y).powi(2) + (p.z - q.z).powi(2)).sqrt()
                / (2.0 * dt);
            max = max.max(d);
            assert!(
                d >= lo * (1.0 - 1e-6),
                "inf: R {major} r {minor} d {offset} t {t}: {d} < {lo}"
            );
        }
        assert!(
            max <= hi * (1.0 + 1e-6),
            "sup: R {major} r {minor} d {offset}: {max} > {hi}"
        );
        eprintln!(
            "r2 spiric R {major} r {minor} d {offset}: inf {lo} sampled max {max} sup {hi} (slack x{})",
            hi / max
        );
        // A whole oval at t0 = 0 and at t0 = 1: the rule's verdict.
        let rows = TurnRows::bounded("r2_turn", "r2_turn_straddle");
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for t0 in [0.0, 1.0, 2.0, 5.0, -3.0, 3.3] {
                let t1 = t0 + TAU;
                let extent = Extent::forward("r2_fwd", t0, t1, InfSpeed::new(lo), band).unwrap();
                let h = TAU - extent.get();
                let sup = turn_defect_speed_hi(&c, t0, h);
                let got = Period::new(TAU).turn(
                    rows,
                    extent,
                    Bounded {
                        inf: InfSpeed::new(lo),
                        sup,
                    },
                    band,
                );
                eprintln!(
                    "r2 spiric whole oval R {major} r {minor} d {offset} eps {eps} t0 {t0}: h {h:e} -> {got:?}"
                );
            }
        }
    }
    // At the tangent limit |d| = R − r the sup is not finite.
    let c = spiric(1.0, 0.5, 0.5);
    let (_, hi) = speed_range(&c);
    eprintln!("r2 spiric at |d| = R - r: sup {}", hi.get());
    let band = Band::new(1e-9, 1e-8).unwrap();
    let extent = Extent::forward("r2_fwd", 0.0, TAU, InfSpeed::new(0.5), band).unwrap();
    let got = Period::new(TAU).turn(
        TurnRows::bounded("r2_turn", "r2_turn_straddle"),
        extent,
        Bounded {
            inf: InfSpeed::new(0.5),
            sup: hi,
        },
        band,
    );
    eprintln!("r2 spiric at |d| = R - r, (0, tau): {got:?}");
}

/// Claim 6: the extent doors. `width` with its arguments swapped, and a
/// bare value through `width(0, x)`.
#[test]
fn r2_width_admits_a_reversed_or_bare_extent() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let reversed = Extent::width(1.0_f64, 0.0);
    assert!(
        reversed.get() < 0.0,
        "width(hi, lo) is negative: {}",
        reversed.get()
    );
    let bare = Extent::width(0.0_f64, -5.0);
    let summed = bare + Extent::width(0.0, 1.0);
    let got = Period::new(TAU).turn(
        TurnRows::exact("r2_turn"),
        summed,
        geom_core::period::Exact(1.0),
        band,
    );
    eprintln!("r2: a -4 extent reads {got:?}");
    assert!(
        matches!(got, Ok(Turn::Short(_))),
        "a negative extent reads Short: {got:?}"
    );
}

/// Claim 6: the sealed trait's hidden `read` takes a bare headroom from
/// any crate, past the `Extent` door.
#[test]
fn r2_turn_rate_read_takes_a_bare_headroom() {
    use geom_core::period::TurnRate as _;
    let band = Band::new(1e-9, 1e-8).unwrap();
    let got = geom_core::period::Exact(1.0_f64).read(TurnRows::exact("r2_turn"), TAU + 5.0, band);
    assert!(matches!(got, Ok(Turn::Short(_))), "{got:?}");
}
