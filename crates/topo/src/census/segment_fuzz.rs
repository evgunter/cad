//! **Pass 5's edge pairs read a meeting as it is**: random segment
//! pairs read by [`interiors_meet`] at `f64` and at [`Interval`], in
//! both orders, at ε = 1e-6, 1e-9 and 1e-12 (K = 10), every reading
//! asserted at both scalars.
//!
//! Three families, each over every angle from far inside the parallel
//! zero to right:
//!
//! - **Crossings**: two segments through one point at least `3Kε` inside
//!   both spans. A definite no is a missed crossing.
//! - **Overlaps**: two segments along one line, overlapping by at least
//!   `3Kε`, every point of the overlap within `ε/3` of the other edge.
//!   A definite no is a missed overlap.
//! - **Crossings past an end**: two segments whose lines cross beyond
//!   one segment's start, that start standing at least `3Kε` off the
//!   other line. A definite yes is a meeting that is not there.
//!
//! An escalation is never wrong here; a definite wrong answer always
//! is. A crossing pair is built in rounded arithmetic, so its rounded
//! ends may cross somewhere other than where it was aimed: each is
//! measured exactly ([`Exact`]) and read only if the real segments
//! still stand `2Kε` inside (or off) their mark. Those dropped are
//! counted and logged, with the escalations.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the readings it checks, and the band and rounding charge
// they decide with.
test_utils::gated_to![
    "crates/topo/src/census.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/k_stats.rs",
    "crates/geom-brep/src/implicit.rs",
];

use super::*;
use geom_core::Interval;
use num_bigint::BigInt;
use test_utils::fuzz::Rng;

const K: f64 = 10.0;
const EPS: [f64; 3] = [1e-6, 1e-9, 1e-12];

type V = [f64; 3];

fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn norm(a: V) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

fn add(a: V, b: V, s: f64) -> V {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}

fn unit(r: &mut Rng) -> V {
    loop {
        let v = [r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0)];
        let n = norm(v);
        if n > 0.1 && n < 1.0 {
            return v.map(|x| x / n);
        }
    }
}

/// A direction at angle `phi` off `a`.
fn turned(r: &mut Rng, a: V, phi: f64) -> V {
    let mut w = cross(a, unit(r));
    while norm(w) < 0.1 {
        w = cross(a, unit(r));
    }
    let w = w.map(|x| x / norm(w));
    add(a.map(|x| x * phi.cos()), w, phi.sin())
}

/// Two segments' lines measured in exact arithmetic, every `f64` an
/// integer count of `2⁻¹⁰⁷⁴`.
struct Exact {
    /// `a`'s and `b`'s chord vectors, and `b`'s start from `a`'s.
    a: [BigInt; 3],
    b: [BigInt; 3],
    d: [BigInt; 3],
    /// `a × b`, and its squared norm.
    n: [BigInt; 3],
    nn: BigInt,
}

/// The exact value of a finite `f64`, in counts of `2⁻¹⁰⁷⁴`.
pub(super) fn big(x: f64) -> BigInt {
    let bits = x.to_bits();
    let exp = ((bits >> 52) & 0x7ff) as usize;
    let frac = bits & ((1u64 << 52) - 1);
    let (mant, shift) = if exp == 0 {
        (frac, 0)
    } else {
        (frac | (1u64 << 52), exp - 1)
    };
    let m = BigInt::from(mant) << shift;
    if bits >> 63 == 1 { -m } else { m }
}

fn big_sub(a: V, b: V) -> [BigInt; 3] {
    [0, 1, 2].map(|i| big(a[i]) - big(b[i]))
}

pub(super) fn big_cross(a: &[BigInt; 3], b: &[BigInt; 3]) -> [BigInt; 3] {
    [
        &a[1] * &b[2] - &a[2] * &b[1],
        &a[2] * &b[0] - &a[0] * &b[2],
        &a[0] * &b[1] - &a[1] * &b[0],
    ]
}

pub(super) fn big_dot(a: &[BigInt; 3], b: &[BigInt; 3]) -> BigInt {
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2]
}

/// `n / d`, `d > 0`, to a few ulps.
pub(super) fn ratio(n: &BigInt, d: &BigInt) -> f64 {
    let head = |x: &BigInt| {
        let shift = x.bits().saturating_sub(62);
        (i64::try_from(x >> shift).unwrap() as f64, shift as i32)
    };
    let ((n, sn), (d, sd)) = (head(n), head(d));
    n / d * 2f64.powi(sn - sd)
}

impl Exact {
    fn of((a0, a1): (V, V), (b0, b1): (V, V)) -> Self {
        let (a, b, d) = (big_sub(a1, a0), big_sub(b1, b0), big_sub(b0, a0));
        let n = big_cross(&a, &b);
        let nn = big_dot(&n, &n);
        Self { a, b, d, n, nn }
    }

    /// Where the lines come closest, per segment the fraction of it
    /// `[from its start, to its end]`: `[a, b]`.
    fn fractions(&self) -> [[f64; 2]; 2] {
        let ta = big_dot(&big_cross(&self.d, &self.b), &self.n);
        let tb = big_dot(&big_cross(&self.d, &self.a), &self.n);
        [ta, tb].map(|t| [ratio(&t, &self.nn), ratio(&(&self.nn - &t), &self.nn)])
    }

    /// The lines' distance, in metres.
    fn gap(&self) -> f64 {
        let dn = big_dot(&self.d, &self.n);
        ratio(&(&dn * &dn), &(&self.nn << 2148)).sqrt()
    }

    /// The distance of `a`'s start from `b`'s line, in metres.
    fn start_off_b(&self) -> f64 {
        let c = big_cross(&self.d, &self.b);
        ratio(&big_dot(&c, &c), &(big_dot(&self.b, &self.b) << 2148)).sqrt()
    }
}

fn meets<T: Decide>((a0, a1): (V, V), (b0, b1): (V, V), band: Band) -> Option<bool> {
    let p = |v: V| Point3::new(T::from_f64(v[0]), T::from_f64(v[1]), T::from_f64(v[2]));
    let mut errors = Vec::new();
    interiors_meet(
        &segment((p(a0), p(a1))),
        &segment((p(b0), p(b1))),
        band,
        &mut errors,
    )
}

/// One built pair, or none.
enum Built {
    /// Two segments, and what built them.
    Pair((V, V), (V, V), String),
    /// The draw was empty, or its rounded ends missed their mark.
    Dropped,
}

/// Each pair built by `pair` at every ε, read in both orders at
/// Interval and at `f64`: the readings that said `wrong`.
fn falsify(label: &str, count: usize, wrong: bool, mut pair: impl FnMut(&mut Rng, f64) -> Built) {
    let mut r = test_utils::fuzz::start(label);
    let mut bad = Vec::new();
    for eps in EPS {
        let band = Band::new(eps, K * eps).unwrap();
        let (mut read, mut dropped, mut f64_held, mut iv_held) = (0usize, 0usize, 0usize, 0usize);
        for _ in 0..test_utils::fuzz::scaled(count) {
            let Built::Pair(a, b, what) = pair(&mut r, eps) else {
                dropped += 1;
                continue;
            };
            for (p, q) in [(a, b), (b, a)] {
                let iv = meets::<Interval>(p, q, band);
                let f = meets::<f64>(p, q, band);
                read += 1;
                f64_held += usize::from(f.is_none());
                iv_held += usize::from(iv.is_none());
                if iv == Some(wrong) || f == Some(wrong) {
                    bad.push(format!(
                        "ε {eps:e} {what}: f64 {f:?}, Interval {iv:?}, a {p:?}, b {q:?}"
                    ));
                }
            }
        }
        println!(
            "[fuzz] {label} at ε {eps:e}: {read} readings, {f64_held} escalated at f64, \
             {iv_held} at Interval; {dropped} draws dropped"
        );
    }
    assert!(
        bad.is_empty(),
        "{label}: {} readings said {wrong}; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(8)].join("\n")
    );
}

/// **Two segments crossing strictly inside both spans never read
/// apart.**
#[test]
fn crossings_strictly_inside_both_spans_never_read_apart() {
    falsify("census_segment_crossings", 500, false, |r, eps| {
        let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
        let a = unit(r);
        let phi = r.log_uniform(1e-14, 10f64.powf(0.2));
        let b = turned(r, a, phi);
        let m = 3.0 * K * eps;
        let la = r.log_uniform(7.0 * m, 10f64.powf(0.5));
        let lb = r.log_uniform(7.0 * m, 10f64.powf(0.5));
        let sa = r.range(m, la - m);
        let sb = r.range(m, lb - m);
        let ea = (add(x, a, -sa), add(x, a, la - sa));
        let eb = (add(x, b, -sb), add(x, b, lb - sb));
        let exact = Exact::of(ea, eb);
        let [ta, tb] = exact.fractions();
        let inside = |[from, to]: [f64; 2], len: f64| from.min(to) * len >= 2.0 * K * eps;
        if !(inside(ta, la) && inside(tb, lb) && exact.gap() <= eps / 2.0) {
            return Built::Dropped;
        }
        Built::Pair(
            ea,
            eb,
            format!("φ {phi:e} la {la:e} lb {lb:e} sa {sa:e} sb {sb:e}"),
        )
    });
}

/// **Two segments overlapping within the zero never read apart.**
#[test]
fn overlaps_within_the_zero_never_read_apart() {
    falsify("census_segment_overlaps", 500, false, |r, eps| {
        let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
        let a = unit(r);
        let m = 3.0 * K * eps;
        let la = r.log_uniform(3.0 * m, 10f64.powf(0.5));
        let lb = r.log_uniform(3.0 * m, 10f64.powf(0.5));
        // `b` starts before `a`'s far end by at least `m`.
        let s = r.range(-lb + m, la - m);
        if la.min(s + lb) - s.max(0.0) < m {
            return Built::Dropped;
        }
        // Its turn and offset keep the overlap within ε/3 of `a`.
        let psi = r.unit() * (eps / 6.0) / la.max(lb);
        let b = turned(r, a, psi);
        let off = cross(a, unit(r));
        let off = off.map(|v| v / norm(off) * (r.unit() * eps / 6.0));
        let b0 = add(add(x, a, s), off, 1.0);
        Built::Pair(
            (x, add(x, a, la)),
            (b0, add(b0, b, lb)),
            format!("ψ {psi:e} la {la:e} lb {lb:e} s {s:e}"),
        )
    });
}

/// **Two segments whose lines cross past one's start, that start
/// standing off the other line, never read as meeting.** Every point
/// of `a` then stands at least as far off `b`'s line as its start.
#[test]
fn crossings_past_an_end_never_read_as_meeting() {
    falsify("census_segment_past_an_end", 500, true, |r, eps| {
        let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
        let a = unit(r);
        let phi = r.log_uniform(1e-14, 10f64.powf(0.2));
        let b = turned(r, a, phi);
        let m = 3.0 * K * eps;
        // `a` starts `o` past the crossing, `o·sin φ ≥ m` off `b`'s line.
        let far = 10f64.powf(0.5);
        let near = m / phi.sin();
        if near >= far {
            return Built::Dropped;
        }
        let o = r.log_uniform(near, far);
        let la = r.log_uniform(7.0 * m, far);
        let lb = r.log_uniform(7.0 * m, far);
        let sb = r.range(m, lb - m);
        let ea = (add(x, a, o), add(x, a, o + la));
        let eb = (add(x, b, -sb), add(x, b, lb - sb));
        let exact = Exact::of(ea, eb);
        if !(exact.fractions()[0][0] <= 0.0 && exact.start_off_b() >= 2.0 * K * eps) {
            return Built::Dropped;
        }
        Built::Pair(
            ea,
            eb,
            format!("φ {phi:e} o {o:e} la {la:e} lb {lb:e} sb {sb:e}"),
        )
    });
}

/// **The closed form's enclosures hold the real values**: for two
/// segments sharing no point key, at any angle, offset and gap, each
/// reach and the gap [`ee_lines_closest`] reads at `f64` encloses the
/// value measured exactly (to its own few ulps).
#[test]
fn the_closed_forms_enclosures_hold_the_exact_values() {
    let mut r = test_utils::fuzz::start("census_lines_closest_enclosures");
    let mut bad = Vec::new();
    let count = test_utils::fuzz::scaled(1500);
    for _ in 0..count {
        let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
        let a = unit(&mut r);
        let phi = r.log_uniform(1e-12, 10f64.powf(0.2));
        let b = turned(&mut r, a, phi);
        let off = cross(a, b);
        let g = if r.unit() < 0.5 {
            0.0
        } else {
            r.log_uniform(1e-15, 1.0)
        };
        let x_b = add(x, off, g / norm(off));
        let la = r.log_uniform(1e-9, 10f64.powf(0.5));
        let lb = r.log_uniform(1e-9, 10f64.powf(0.5));
        let sa = r.range(-la, 2.0 * la);
        let sb = r.range(-lb, 2.0 * lb);
        let ea = (add(x, a, -sa), add(x, a, la - sa));
        let eb = (add(x_b, b, -sb), add(x_b, b, lb - sb));
        let exact = Exact::of(ea, eb);
        let [ta, tb] = exact.fractions();
        let p = |v: V| Point3::new(v[0], v[1], v[2]);
        let (ga, gb) = (segment((p(ea.0), p(ea.1))), segment((p(eb.0), p(eb.1))));
        let read = ee_lines_closest(&ga, &gb);
        let truth = [
            ("gap", exact.gap(), read.gap),
            ("a from its start", ta[0] * ga.len, read.reach[0][0]),
            ("a to its end", ta[1] * ga.len, read.reach[0][1]),
            ("b from its start", tb[0] * gb.len, read.reach[1][0]),
            ("b to its end", tb[1] * gb.len, read.reach[1][1]),
        ];
        for (what, real, [lo, hi]) in truth {
            let slack = 8.0 * f64::EPSILON * real.abs();
            if !(lo - slack <= real && real <= hi + slack) {
                bad.push(format!(
                    "{what} {real:e} outside [{lo:e}, {hi:e}]: φ {phi:e} gap {g:e}, a {ea:?}, b {eb:?}"
                ));
            }
        }
    }
    println!("[fuzz] census_lines_closest_enclosures: {count} pairs");
    assert!(
        bad.is_empty(),
        "{} readings outside their enclosure; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(8)].join("\n")
    );
}

/// **Each single reading's error stays inside half its charge**, the
/// margin [`ee_lines_closest`] claims, over regimes the enclosure test
/// does not sample: coordinates near 1e4, edges up to 1e3 long, ends up
/// to 1e3 lengths off the crossing, φ down to 1e-14. Each reading is the
/// census's own ([`gap_from_ends`], [`reach_from_end`]). Prints the worst
/// ratio of error to charge per regime.
#[test]
fn each_readings_error_stays_inside_its_charge_far_from_the_unit_box() {
    const HALF: f64 = 0.5;
    let mut r = test_utils::fuzz::start("census_lines_closest_charge_ratio");
    let count = test_utils::fuzz::scaled(800);
    for (regime, scale, lmax, far, gmax) in [
        ("unit", 2.0, 3.0, 2.0, 1.0),
        ("coords1e4", 1e4, 3.0, 2.0, 1.0),
        ("coords1e4_long", 1e4, 1e3, 2.0, 1e2),
        ("ends_far", 2.0, 3.0, 100.0, 1.0),
        ("coords1e4_ends_far", 1e4, 30.0, 1000.0, 1e3),
    ] {
        let mut worst = [(0.0f64, String::new()), (0.0, String::new())];
        let mut over = 0usize;
        for _ in 0..count {
            let x = [0, 1, 2].map(|_| r.range(-scale, scale));
            let a = unit(&mut r);
            let phi = r.log_uniform(1e-14, 1.0);
            let b = turned(&mut r, a, phi);
            let off = cross(a, b);
            let g = if r.unit() < 0.3 {
                0.0
            } else {
                r.log_uniform(1e-15, gmax)
            };
            let x_b = add(x, off, g / norm(off));
            let la = r.log_uniform(1e-6, lmax);
            let lb = r.log_uniform(1e-6, lmax);
            let sa = r.range(-far * la, far * la);
            let sb = r.range(-far * lb, far * lb);
            let ea = (add(x, a, -sa), add(x, a, la - sa));
            let eb = (add(x_b, b, -sb), add(x_b, b, lb - sb));
            let exact = Exact::of(ea, eb);
            if exact.nn == BigInt::from(0) {
                continue;
            }
            let [ta, tb] = exact.fractions();
            let real_gap = exact.gap();
            let p = |v: V| Point3::new(v[0], v[1], v[2]);
            let (ga, gb) = (segment((p(ea.0), p(ea.1))), segment((p(eb.0), p(eb.1))));
            let (n, nn, s) = lines_frame(&ga, &gb);
            // A reading's error over its charge, the real value's own few
            // ulps aside: past 1, the enclosure misses it.
            let ratio = |[lo, hi]: [f64; 2], real: f64| {
                let miss = ((lo + hi) / 2.0 - real).abs() - 4.0 * f64::EPSILON * real.abs();
                miss.max(0.0) / ((hi - lo) / 2.0)
            };
            let mut hi = f64::INFINITY;
            for (pp, qq) in [
                (ga.p0, gb.p0),
                (ga.p0, gb.p1),
                (ga.p1, gb.p0),
                (ga.p1, gb.p1),
            ] {
                let read = gap_from_ends(qq - pp, n, s);
                hi = hi.min(read[1]);
                let ratio = ratio(read, real_gap);
                if ratio > worst[0].0 {
                    worst[0] = (ratio, format!("phi {phi:e} g {g:e} la {la:e} sa {sa:e}"));
                }
                over += usize::from(ratio > HALF);
            }
            for (side, (xx, yy, t)) in [(&ga, &gb, ta), (&gb, &ga, tb)].into_iter().enumerate() {
                let w = reach_axis(xx, yy, nn);
                for (k, pp) in [xx.p0, xx.p1].into_iter().enumerate() {
                    let real = t[k] * xx.len;
                    for qq in [yy.p0, yy.p1] {
                        let ratio = ratio(reach_from_end(qq - pp, w, k == 1, hi, s), real);
                        if ratio > worst[1].0 {
                            worst[1] = (
                                ratio,
                                format!(
                                    "side {side} k {k} phi {phi:e} g {g:e} la {la:e} lb {lb:e} sa {sa:e} sb {sb:e}"
                                ),
                            );
                        }
                        over += usize::from(ratio > HALF);
                    }
                }
            }
        }
        println!(
            "[fuzz] census_lines_closest_charge_ratio {regime}: worst gap ratio {:.3} ({}), worst reach ratio {:.3} ({}), over {over}",
            worst[0].0, worst[0].1, worst[1].0, worst[1].1
        );
        assert_eq!(
            over, 0,
            "{regime}: a reading's error exceeded half its charge"
        );
    }
}

/// **A reach whose enclosure straddles the band is never read as
/// interior**: two nearly parallel segments
/// whose lines cross `2Kε` before `a`'s start, `b`'s far end beside
/// `a`'s far end (so the gap's enclosure is tight and reads Zero),
/// while `a`'s start stands half a metre from both of `b`'s ends (so
/// its reach's enclosure straddles the band). The crossing is outside
/// `a`'s span: a definite yes is wrong; an escalation is right.
#[test]
fn a_span_straddle_past_an_end_is_never_a_crossing() {
    let eps = 1e-12;
    let band = Band::new(eps, K * eps).unwrap();
    let x = [0.3, 0.2, 0.1];
    let a = [1.0, 0.0, 0.0];
    for phi in [1e-5, 2e-5, 4e-5] {
        let b = [phi.cos(), phi.sin(), 0.0];
        let o = 2.0 * K * eps;
        let ea = (add(x, a, o), add(x, a, o + 1.0));
        let eb = (add(x, b, -0.5), add(x, b, o + 1.0));
        let exact = Exact::of(ea, eb);
        assert!(
            exact.fractions()[0][0] < 0.0,
            "φ {phi:e}: the crossing is before a's start"
        );
        for (p, q) in [(ea, eb), (eb, ea)] {
            let f = meets::<f64>(p, q, band);
            let iv = meets::<Interval>(p, q, band);
            assert_ne!(
                f,
                Some(true),
                "φ {phi:e}: f64 read a crossing past a's start"
            );
            assert_ne!(
                iv,
                Some(true),
                "φ {phi:e}: Interval read a crossing past a's start"
            );
        }
    }
}

/// **A crossing beside two near ends is decided from them**: two
/// segments at φ crossing `t` before both their ends, `a` starting a
/// metre back and `b` a centimetre back. Read from either start, or
/// from `a`'s start to `b`'s end, the gap carries about
/// `rounding_charge(1 m)/sin φ`, over ε = 1e-12 and under `Kε` at every
/// φ here (8.9e-12 to 1.8e-12), so it escalates; read from the ends,
/// `t·φ` apart, it carries next to nothing. Only the intersection of
/// every reading decides it.
#[test]
fn a_crossing_beside_two_near_ends_is_decided_from_them() {
    let eps = 1e-12;
    let band = Band::new(eps, K * eps).unwrap();
    let x = [0.3, 0.2, 0.1];
    let a = [1.0, 0.0, 0.0];
    let t = 1e-6;
    let mut undecided = Vec::new();
    for phi in [2e-4, 5e-4, 1e-3] {
        let b = [phi.cos(), phi.sin(), 0.0];
        let ea = (add(x, a, -1.0), add(x, a, t));
        let eb = (add(x, b, -0.01), add(x, b, t));
        let exact = Exact::of(ea, eb);
        let lens = [1.0 + t, 0.01 + t];
        let inside = exact
            .fractions()
            .iter()
            .zip(lens)
            .all(|([from, to], len)| from.min(*to) * len >= 2.0 * K * eps);
        assert!(
            inside && exact.gap() <= eps / 2.0,
            "φ {phi:e}: the real segments cross inside both spans"
        );
        for (p, q) in [(ea, eb), (eb, ea)] {
            let (f, iv) = (meets::<f64>(p, q, band), meets::<Interval>(p, q, band));
            if (f, iv) != (Some(true), Some(true)) {
                undecided.push(format!("φ {phi:e}: f64 {f:?}, Interval {iv:?}"));
            }
        }
    }
    assert!(
        undecided.is_empty(),
        "each scalar decides the crossing from the near ends:\n{}",
        undecided.join("\n")
    );
}

/// **Lines apart by a gap their enclosure straddles never read as
/// meeting**: two nearly parallel segments crossing in plan inside
/// both spans, their lines `2Kε` apart, every end at least half a metre
/// from every other. Each reading of the gap carries over `2Kε` of
/// charge, so `lo` reads Zero and `hi` Positive: the gap straddles, and
/// the pair escalates. A Zero there is a crossing that is not there.
#[test]
fn a_gap_straddle_on_lines_apart_escalates() {
    let eps = 1e-12;
    let band = Band::new(eps, K * eps).unwrap();
    let x = [0.3, 0.2, 0.1];
    let a = [1.0, 0.0, 0.0];
    let up = [0.0, 0.0, 2.0 * K * eps];
    for phi in [1e-5, 2e-5, 4e-5] {
        let b = [phi.cos(), phi.sin(), 0.0];
        let xb = add(x, up, 1.0);
        let ea = (add(x, a, -1.0), add(x, a, 0.5));
        let eb = (add(xb, b, -0.5), add(xb, b, 1.0));
        let exact = Exact::of(ea, eb);
        let inside = exact
            .fractions()
            .iter()
            .all(|[from, to]| from.min(*to) * 1.5 >= 0.4);
        assert!(
            inside && exact.gap() >= 1.5 * K * eps,
            "φ {phi:e}: the real lines pass over Kε apart inside both spans"
        );
        for (p, q) in [(ea, eb), (eb, ea)] {
            let (f, iv) = (meets::<f64>(p, q, band), meets::<Interval>(p, q, band));
            assert_eq!(
                (f, iv),
                (None, None),
                "φ {phi:e}: the gap's straddle escalates at both scalars"
            );
        }
    }
}
