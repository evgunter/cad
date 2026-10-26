//! **The turn rule against the true arc length of an ellipse's turn
//! defect** (review R2 of PR 5686's rows, joined to the suite):
//! `turn_defect_speed_hi` against the old `ConicArc` expression and
//! against sampled speeds, both defect bounds against certified speeds
//! over the true defect at `f64` and `Interval`, the rule's verdicts
//! against the defect's metric length, and the spiric's sup over the
//! defect, near tangency and far along the parameter.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/carrier_speed.rs"];

use geom::Curve3;
use geom::carrier_speed::{speed_range, turn_defect_speed_hi, turn_defect_speed_lo};
use geom_core::period::{Bounded, Extent, Period, Turn, TurnRows};
use geom_core::{Band, Bounds, InfSpeed, Interval, Point3, Real, SupSpeed, Vec3};
use test_utils::fuzz;

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

/// Arc length over the stretch `width` wide about `mid`, composite
/// Simpson. The step is read off `width`, not off the difference of two
/// rounded ends, which at a defect of `1e-13` about `θ = 4` is a whole
/// part in a few hundred.
fn arc(a: f64, b: f64, mid: f64, width: f64) -> f64 {
    let n = 256;
    let h = width / f64::from(n);
    let at = |i: i32| speed(a, b, mid + width * (f64::from(i) / f64::from(n) - 0.5));
    let mut s = at(0) + at(n);
    for i in 1..n {
        let w = if i % 2 == 1 { 4.0 } else { 2.0 };
        s += w * at(i);
    }
    s * h / 3.0
}

/// Claim 2, bits: the old `ConicArc` sup expression, verbatim.
fn old_sup<T: geom_core::Real>(a: T, b: T, t0: T, gap: T) -> T {
    let (sm, cm) = (t0 - gap * T::from_f64(0.5)).sin_cos();
    ((a * sm).powi(2) + (b * cm).powi(2)).sqrt()
        + a.abs().max(b.abs()) * gap.abs() * T::from_f64(0.5)
}

/// Claim 2, bits: the old sup moved by no more than the reach's rounding
/// allowance (`2⁻⁴⁶` of `1 + |m|`, times the larger semi-axis), and only
/// up, or down to the larger semi-axis it is now held to; and it bounds
/// the sampled speed over the defect with no slack of its own.
#[test]
fn r2_turn_defect_speed_hi_is_the_old_sup_widened_by_its_rounding_and_an_upper_bound() {
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
        let extent =
            Extent::hull([0.0, TAU - (rnd() * 2.0 - 1.0) * (rnd() * 30.0 - 25.0).exp()]).unwrap();
        let gap = TAU - extent.get();
        let got = turn_defect_speed_hi(&ellipse(a, b), t0, extent).get();
        let m = t0 - 0.5 * gap;
        let ceiling = a.abs().max(b.abs());
        let old = old_sup(a, b, t0, gap);
        let allowance = ceiling * (1.0 + m.abs()) / (1u64 << 46) as f64;
        let case = format!("a {a} b {b} t0 {t0} gap {gap}");
        assert!(
            got >= old.min(ceiling) && got <= old + allowance + 4.0 * f64::EPSILON * got,
            "moved by more than the allowance {allowance:e}: {got} against {old}: {case}"
        );
        // Upper bound over the defect: middle t0 - gap/2, width |gap|.
        for i in 0..=64 {
            let t = m + gap.abs() * (f64::from(i) / 64.0 - 0.5);
            let s = speed(a, b, t);
            worst = worst.max((s - got) / got);
            assert!(s <= got, "speed {s} > sup {got}: {case}");
        }
    }
    eprintln!("r2: worst (sampled - sup)/sup = {worst:e}");
}

/// Claim 3: the rule's verdicts, at the inf and the sup over the defect,
/// against the true metric length of the defect, for eccentric ellipses
/// on the fast, slow and in-between side.
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
                            let low = turn_defect_speed_lo(&ellipse(a, b), t0, extent);
                            let sup = turn_defect_speed_hi(&ellipse(a, b), t0, extent);
                            let got = one.turn(rows, extent, Bounded::new(low, sup), band);
                            // The defect stretch, centred at t0 - h/2.
                            let mid = t0 - 0.5 * h;
                            // The defect as the rule reads it at f64, `τ − extent`
                            // wide: f64 τ's distance from 2π is not the rule's.
                            let len = arc(a, b, mid, h.abs());
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
                                    if h < 0.0 && !u.over_at_sup() {
                                        assert!(
                                            len <= 10.0 * eps * slack,
                                            "over_at_sup false but overlap past the band: {case}"
                                        );
                                    }
                                    if u.over_at_sup() {
                                        assert!(h < 0.0, "{case}");
                                    }
                                    // And of its twin, the one admission
                                    // "not over" makes of an undecided
                                    // reading: the extent is short.
                                    if u.not_over_at_sup() {
                                        assert!(h > 0.0, "not over at the sup: {case}");
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

/// Claim 3, spiric: the period's sup is finite for a whole oval
/// (|d| < R − r) and an upper bound on the sampled speed; the sup over a
/// whole oval's defect is within it, and reads the oval a whole turn at
/// every ε; and at the tangent limit the period's sup settles nothing.
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
        let sup2 = turn_defect_speed_hi(&c, 0.3, Extent::hull([0.0, TAU - 1e-9]).unwrap()).get();
        assert!(
            sup2 <= hi,
            "spiric defect sup {sup2} within the period's {hi}: R {major} r {minor} d {offset}"
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
                let sup = turn_defect_speed_hi(&c, t0, extent);
                let got =
                    Period::new(TAU).turn(rows, extent, Bounded::new(InfSpeed::new(lo), sup), band);
                // The sup over the defect is local, so a rounding-sized
                // headroom stays in the zero band however near tangency:
                // at `d = R − r − 1e-12`, ε = 1e-12, `t0 = 3.3` escalated
                // at the period's sup.
                assert!(
                    matches!(got, Ok(Turn::Whole(_))),
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
        Bounded::new(InfSpeed::new(0.5), SupSpeed::new(hi.get())),
        band,
    );
    // A sup that is not finite settles no whole turn: it escalates, on an
    // unreadable margin, never `Whole`.
    assert!(
        matches!(got, Err(ref u) if !u.over_at_sup() && !u.not_over_at_sup()),
        "r2 spiric at |d| = R - r, (0, tau): {got:?}"
    );
    // The sup over the defect is finite where the defect is clear of the
    // pinch (`v = π`), and settles the whole oval there; on the pinch it
    // is not, and settles nothing.
    let rows = TurnRows::bounded("r2_turn", "r2_turn_straddle");
    for (t0, whole) in [(0.0, true), (1.0, true), (core::f64::consts::PI, false)] {
        let extent = Extent::forward("r2_fwd", t0, t0 + TAU, InfSpeed::new(0.5), band).unwrap();
        let sup = turn_defect_speed_hi(&c, t0, extent);
        let got = Period::new(TAU).turn(rows, extent, Bounded::new(InfSpeed::new(0.5), sup), band);
        assert_eq!(
            matches!(got, Ok(Turn::Whole(_))),
            whole,
            "r2 spiric at |d| = R - r, t0 {t0}: sup {} -> {got:?}",
            sup.get()
        );
        if !whole {
            assert!(
                matches!(got, Err(ref u) if !u.over_at_sup() && !u.not_over_at_sup()),
                "r2 spiric at |d| = R - r, on the pinch: {got:?}"
            );
        }
    }
}

/// `(sin, cos)` of `t0 + d`, `t0` an exact `f64` and `d` an enclosure,
/// through the addition formula, so a far `t0` keeps a tight enclosure.
fn sin_cos_at(t0: f64, d: Interval) -> (Interval, Interval) {
    let (s0, c0) = Interval::from_f64(t0).sin_cos();
    let (sd, cd) = d.sin_cos();
    (s0 * cd + c0 * sd, c0 * cd - s0 * sd)
}

/// The **true** turn defect of the window `t0` to `t0 + x`, as offsets
/// from `t0`: the stretch `hull(0, x − 2π)`, `2π` the real number, so
/// f64 τ's distance from it is the defect's and not the bound's.
fn true_defect(x: f64) -> Interval {
    Interval::from_f64(x) - Interval::tau()
}

/// The speed of an ellipse at `t0 + d`, as a certified enclosure: its
/// ends bracket the true speed.
fn speed_enclosure(a: f64, b: f64, t0: f64, d: Interval) -> Interval {
    let (s, c) = sin_cos_at(t0, d);
    ((Interval::from_f64(a) * s).powi(2) + (Interval::from_f64(b) * c).powi(2)).sqrt()
}

/// **The turn defect's two speed bounds hold over the true defect, at
/// `f64` and at `Interval`, and are local** (a counterexample search: the
/// seed varies): on ellipses of every sign and of eccentricity up to
/// `e¹⁰ ≈ 2·10⁴`, a window at any `t0` out to `10⁶` (where the reach's
/// rounding allowance is widest) short of a turn or over one by a defect
/// from `1e-15` to two radians, the inf at either scalar is at most the
/// certified speed's lower end, and the sup at least its upper end, at 65
/// points across the true defect; and each is within the larger semi-axis
/// times the defect's width (plus rounding) of the least, or greatest,
/// speed sampled there, so neither is a global bound in disguise.
#[test]
fn turn_defect_speeds_bound_the_speed_on_the_true_defect_at_f64_and_interval() {
    let mut rng = fuzz::start("r2_turn_probes::turn_defect_speeds");
    for _ in 0..fuzz::scaled(3000) {
        let sign = |rng: &mut fuzz::Rng, p: f64| if rng.unit() < p { -1.0 } else { 1.0 };
        let a = rng.range(-5.0, 5.0).exp() * sign(&mut rng, 0.2);
        let b = rng.range(-5.0, 5.0).exp() * sign(&mut rng, 0.1);
        let t0 = if rng.unit() < 0.3 {
            rng.range(-20.0, 20.0)
        } else {
            rng.log_uniform(20.0, 1e6) * sign(&mut rng, 0.5)
        };
        let gap = rng.log_uniform(1e-15, 2.0) * sign(&mut rng, 0.5);
        let x = TAU - gap;
        let carrier = ellipse(a, b);
        let carrier_iv = carrier.map_scalar(Interval::from_f64);
        let extent = Extent::hull([0.0, x]).unwrap();
        let extent_iv = Extent::hull([Interval::zero(), Interval::from_f64(x)]).unwrap();
        let lo = turn_defect_speed_lo(&carrier, t0, extent).get();
        let hi = turn_defect_speed_hi(&carrier, t0, extent).get();
        let t0_iv = Interval::from_f64(t0);
        let lo_iv = turn_defect_speed_lo(&carrier_iv, t0_iv, extent_iv)
            .get()
            .lo();
        let hi_iv = turn_defect_speed_hi(&carrier_iv, t0_iv, extent_iv)
            .get()
            .hi();
        let (floor, ceiling) = (a.abs().min(b.abs()), a.abs().max(b.abs()));
        let case = format!("a {a} b {b} t0 {t0} gap {gap:e} ({})", fuzz::replay());
        assert!(
            lo >= floor && lo_iv >= floor * (1.0 - 1e-15),
            "never below the floor: {lo} / {lo_iv} < {floor}: {case}"
        );
        assert!(
            hi <= ceiling && hi_iv <= ceiling * (1.0 + 1e-15),
            "never above the ceiling: {hi} / {hi_iv} > {ceiling}: {case}"
        );
        let defect = true_defect(x);
        let (mut least, mut most) = (f64::INFINITY, 0.0_f64);
        for i in 0..=64 {
            let d = defect * Interval::from_f64(f64::from(i) / 64.0);
            let speed = speed_enclosure(a, b, t0, d);
            least = least.min(speed.lo());
            most = most.max(speed.hi());
            assert!(
                lo <= speed.lo() && lo_iv <= speed.lo(),
                "inf over the defect: {lo} (f64) / {lo_iv} (Interval) > {} at {i}/64: {case}",
                speed.lo()
            );
            assert!(
                hi >= speed.hi() && hi_iv >= speed.hi(),
                "sup over the defect: {hi} (f64) / {hi_iv} (Interval) < {} at {i}/64: {case}",
                speed.hi()
            );
        }
        let m = t0 - 0.5 * gap;
        let reach = ceiling * gap.abs() + ceiling * (1.0 + m.abs()) * 1e-13;
        assert!(
            lo + reach >= least && hi - reach <= most,
            "local: [{lo}, {hi}] ± {reach:e} against [{least}, {most}] sampled: {case}"
        );
    }
}

/// The speed of a spiric at `v`, `r·√(1 + (d·sin v / f)²)` with
/// `f = √((R + r·cos v)² − d²)`, as a certified enclosure.
fn spiric_speed_enclosure(major: f64, minor: f64, offset: f64, v: f64) -> Interval {
    let (major, minor, offset) = (
        Interval::from_f64(major),
        Interval::from_f64(minor),
        Interval::from_f64(offset),
    );
    let (s, c) = Interval::from_f64(v).sin_cos();
    let f = ((major + minor * c).powi(2) - offset.powi(2)).sqrt();
    minor * (Interval::one() + (offset * s / f).powi(2)).sqrt()
}

/// **A spiric's sup over the turn defect is an upper bound there, and is
/// local**: on whole ovals from far off tangency to `1e-12` of it, at
/// defect centres on the pinch (`v = π`), beside it and away from it, the
/// sup at `Interval` is at least the certified speed's upper end at 65
/// points across the defect (at `f64`, its lower end: the same
/// expressions in round-to-nearest, `speed_range`'s contract), at most
/// the whole period's sup, and — on a defect of `1e-6` clear of the
/// pinch — within 1% of the greatest speed sampled there, where the
/// period's sup is up to `×354` the sampled speed.
#[test]
fn a_spirics_defect_sup_bounds_the_speed_there_and_is_local() {
    let spiric = |major: f64, minor: f64, offset: f64| Curve3::Spiric {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        major_radius: major,
        minor_radius: minor,
        offset,
    };
    let pi = core::f64::consts::PI;
    for (major, minor, offset) in [
        (1.0, 0.5, 0.0),
        (1.0, 0.5, 0.25),
        (1.0, 0.5, 0.5 - 1e-6),
        (1.0, 0.5, 0.5 - 1e-12),
        (3.0, 1.0, -1.5),
    ] {
        let c = spiric(major, minor, offset);
        let c_iv = c.map_scalar(Interval::from_f64);
        let period_sup = speed_range(&c).1.get();
        for centre in [0.0, 1.0, 2.5, pi - 0.1, pi - 1e-4, pi, 4.0] {
            for gap in [0.0, 1e-12, -1e-9, 1e-6, -1e-3, 0.1, -1.0] {
                let x = TAU - gap;
                let t0 = centre + 0.5 * gap;
                let hi = turn_defect_speed_hi(&c, t0, Extent::hull([0.0, x]).unwrap()).get();
                let hi_iv = turn_defect_speed_hi(
                    &c_iv,
                    Interval::from_f64(t0),
                    Extent::hull([Interval::zero(), Interval::from_f64(x)]).unwrap(),
                )
                .get()
                .hi();
                let case = format!("R {major} r {minor} d {offset} centre {centre} gap {gap:e}");
                assert!(
                    hi <= period_sup,
                    "within the period's sup {period_sup}: {hi}: {case}"
                );
                let mut most: f64 = 0.0;
                for i in 0..=64 {
                    let v = t0 + x + gap * (f64::from(i) / 64.0);
                    let speed = spiric_speed_enclosure(major, minor, offset, v);
                    most = most.max(speed.hi());
                    assert!(
                        hi_iv >= speed.hi() && hi >= speed.lo(),
                        "sup over the defect: {hi_iv} (Interval) < {} or {hi} (f64) < {} at v \
                         {v}: {case}",
                        speed.hi(),
                        speed.lo()
                    );
                }
                if gap.abs() <= 1e-6 && (centre - pi).abs() >= 0.1 {
                    assert!(
                        hi <= most * 1.01,
                        "local: {hi} against the most sampled {most}: {case}"
                    );
                }
            }
        }
    }
}

/// **A spiric's sup over the turn defect holds at `f64` far along the
/// parameter and on the steep flank of a near-tangent pinch**: the guard
/// on the reach's rounding allowance. Within `2·10⁻⁶` of the pinch of
/// `(1, 0.5, 0.5 − 10⁻¹²)` the speed's relative slope is about `5·10⁵`
/// per radian, so the `f64` defect's distance from the true one (f64 τ's
/// `2.4·10⁻¹⁶` from 2π, and the rounding of its middle, a few units of
/// `t0` out to `6·10⁵`) moves the speed by more than the sup's own
/// rounding; without the allowance the `f64` sup under-reads the
/// certified speed on the true defect here.
#[test]
fn a_spirics_defect_sup_holds_at_f64_far_along_and_beside_the_pinch() {
    let c = Curve3::Spiric {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        major_radius: 1.0,
        minor_radius: 0.5,
        offset: 0.5 - 1e-12,
    };
    let c_iv = c.map_scalar(Interval::from_f64);
    let (major, minor, offset) = (1.0, 0.5, 0.5 - 1e-12);
    let pi = core::f64::consts::PI;
    let mut checked = 0;
    for turns in [0.0, 1.0, 1e3, 1e5, -3e4, 9.5e4] {
        for delta in [-2e-6, -1e-6, -4e-6, 1e-6, 2e-6, 4e-6] {
            for gap in [0.0, 1e-15, -1e-15, 1e-13, -1e-13, 3e-10, -3e-10] {
                // A centre `delta` off the pinch, `turns` periods on (to
                // `6·10⁵`); the window starts half its headroom past it.
                let centre = turns * TAU + pi + delta;
                let x = TAU - gap;
                let t0 = centre + 0.5 * gap;
                let hi = turn_defect_speed_hi(&c, t0, Extent::hull([0.0, x]).unwrap()).get();
                let hi_iv = turn_defect_speed_hi(
                    &c_iv,
                    Interval::from_f64(t0),
                    Extent::hull([Interval::zero(), Interval::from_f64(x)]).unwrap(),
                )
                .get()
                .hi();
                let defect = true_defect(x);
                for i in 0..=32 {
                    let d = defect * Interval::from_f64(f64::from(i) / 32.0);
                    let (s, cs) = sin_cos_at(t0, d);
                    let (major, minor, offset) = (
                        Interval::from_f64(major),
                        Interval::from_f64(minor),
                        Interval::from_f64(offset),
                    );
                    let f = ((major + minor * cs).powi(2) - offset.powi(2)).sqrt();
                    let speed = minor * (Interval::one() + (offset * s / f).powi(2)).sqrt();
                    assert!(
                        hi >= speed.hi() && hi_iv >= speed.hi(),
                        "sup over the true defect: {hi} (f64) / {hi_iv} (Interval) < {:?} at \
                         {i}/32: t0 {t0} delta {delta:e} gap {gap:e}",
                        speed
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 0);
}
