//! **RING-0 review probes (lane R1).** Hand-built witnesses for every
//! class the ring differential's allowlist names, at both oracles; the
//! division corners the survey asked about; the exactness of each class
//! predicate (does it admit inputs on which the two arithmetics AGREE?);
//! and the expected hits per class per lane at `CAD_FUZZ_EFFORT=1`,
//! computed from the generator's own distribution rather than sampled.
//!
//! The predicates are re-spelled here from
//! `ring_interval_differential.rs` (they are private to that file); the
//! copy is the point — a probe that imported them would test the
//! predicate against itself.

test_utils::gated_to![
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-core/src/interval.rs",
    "interval-transcendentals/src/",
];

use geom_core::RingInterval;
use interval_transcendentals::{DInterval, Decoration};

const INF: f64 = f64::INFINITY;
const NINF: f64 = f64::NEG_INFINITY;

fn ri(lo: f64, hi: f64) -> RingInterval {
    RingInterval::from_bounds(lo, hi)
}

fn di(lo: f64, hi: f64) -> DInterval {
    DInterval::from_bounds(lo, hi)
}

/// The backend's refusal, as the differential spells it.
fn d_refuses(d: DInterval) -> bool {
    d.is_nai() || d.is_empty() || d.decoration() < Decoration::Def
}

#[cfg(feature = "interval")]
fn i_refuses(
    lo: f64,
    hi: f64,
    f: impl Fn(geom_core::Interval, geom_core::Interval) -> geom_core::Interval,
    blo: f64,
    bhi: f64,
) -> bool {
    let a = geom_core::Interval::from_bounds(lo, hi);
    let b = geom_core::Interval::from_bounds(blo, bhi);
    !f(a, b).is_certified()
}

// ------------------------------------------------- class 1: 0 · inf

#[test]
fn mul_zero_times_infinite_by_hand() {
    // [0, 1] · [1, +inf]: the ring forms the corner 0·inf = NaN and
    // poisons; the backend's mul_lo/mul_hi answer 0 for the zero factor
    // and keep Dac on [0, +inf].
    let r = ri(0.0, 1.0) * ri(1.0, INF);
    let d = di(0.0, 1.0) * di(1.0, INF);
    assert!(r.is_poison());
    assert!(!d_refuses(d));
    assert_eq!(d.decoration(), Decoration::Dac);
    assert_eq!((d.lo(), d.hi()), (0.0, INF));
    // Signed zero on the other side, infinite side negative: same fact.
    let r = ri(-INF, -1.0) * ri(-0.0, 1.0);
    let d = di(-INF, -1.0) * di(-0.0, 1.0);
    assert!(r.is_poison() && !d_refuses(d));
    // The exact zero is answered by the annihilator first on the ring and
    // by the zero convention on the backend: an AGREEMENT, which is why
    // the predicate excludes it.
    let r = ri(0.0, 0.0) * ri(1.0, INF);
    let d = di(0.0, 0.0) * di(1.0, INF);
    assert!(!r.is_poison() && !d_refuses(d));
    assert_eq!((r.lo(), r.hi()), (0.0, 0.0));
    assert_eq!((d.lo(), d.hi()), (0.0, 0.0));
    // A zero endpoint against a bounded factor: no corner, agreement.
    let r = ri(0.0, 1.0) * ri(1.0, 2.0);
    let d = di(0.0, 1.0) * di(1.0, 2.0);
    assert!(!r.is_poison() && !d_refuses(d));
    // An infinite endpoint against a zero-FREE factor: agreement.
    let r = ri(1.0, INF) * ri(-1.0, 2.0);
    let d = di(1.0, INF) * di(-1.0, 2.0);
    assert!(!r.is_poison() && !d_refuses(d));
}

#[cfg(feature = "interval")]
#[test]
fn mul_zero_times_infinite_by_hand_at_the_interval_scalar() {
    assert!(!i_refuses(0.0, 1.0, |a, b| a * b, 1.0, INF));
    assert!(!i_refuses(-INF, -1.0, |a, b| a * b, -0.0, 1.0));
    assert!(!i_refuses(0.0, 0.0, |a, b| a * b, 1.0, INF));
}

// ------------------------------------------- class 2: inf / inf

#[test]
fn div_infinite_over_infinite_by_hand() {
    // [1, +inf] / [1, +inf]: the ring's corner inf/inf is NaN and it
    // poisons; the backend's f64::min/max fold drops the NaN corner and
    // answers [0, +inf] at Dac.
    let r = ri(1.0, INF) / ri(1.0, INF);
    let d = di(1.0, INF) / di(1.0, INF);
    assert!(r.is_poison());
    assert!(!d_refuses(d));
    assert_eq!(d.decoration(), Decoration::Dac);
    // The backend pads 1/inf = 0 down one step (no exactness witness at
    // an infinite factor), so its lower end is -5e-324, not 0.
    assert!(d.lo() <= 0.0 && d.lo() >= -5e-324 && d.hi() == INF, "{d:?}");
    // Both unbounded, divisor negative: same.
    let r = ri(-INF, 1.0) / ri(-INF, -1.0);
    let d = di(-INF, 1.0) / di(-INF, -1.0);
    assert!(r.is_poison() && !d_refuses(d));
    // Only the numerator unbounded: no NaN corner, agreement.
    let r = ri(1.0, INF) / ri(1.0, 2.0);
    let d = di(1.0, INF) / di(1.0, 2.0);
    assert!(!r.is_poison() && !d_refuses(d));
    // Only the divisor unbounded: 1/inf = 0, agreement.
    let r = ri(1.0, 2.0) / ri(1.0, INF);
    let d = di(1.0, 2.0) / di(1.0, INF);
    assert!(!r.is_poison() && !d_refuses(d));
    // The exact-zero numerator over an unbounded divisor: agreement (the
    // ring's zero rule; the backend's div_lo(0, ·) = 0).
    let r = ri(0.0, 0.0) / ri(1.0, INF);
    let d = di(0.0, 0.0) / di(1.0, INF);
    assert!(!r.is_poison() && !d_refuses(d));
}

#[cfg(feature = "interval")]
#[test]
fn div_infinite_over_infinite_by_hand_at_the_interval_scalar() {
    assert!(!i_refuses(1.0, INF, |a, b| a / b, 1.0, INF));
    assert!(!i_refuses(-INF, 1.0, |a, b| a / b, -INF, -1.0));
}

// ------------------------------------------- class 3: powi's multiply

#[test]
fn powi_chain_by_hand() {
    // The PR's smallest witness: [-MAX, -0.0]^3. The ring's chain is
    // result = x, acc = x.sqr() = [0, +inf], result * acc forms -0·inf.
    let x = ri(-f64::MAX, -0.0);
    assert!(x.powi(3).is_poison());
    let d = di(-f64::MAX, -0.0).powi(3);
    assert!(!d_refuses(d));
    assert_eq!((d.lo(), d.hi()), (NINF, 0.0));
    // n = 2 and n = 4 only square: no multiply, no corner, agreement.
    assert!(!x.powi(2).is_poison() && !x.powi(4).is_poison());
    assert!(!d_refuses(di(-f64::MAX, -0.0).powi(4)));
    // Negative exponent: the positive power [-inf, 0] touches zero, so
    // the reciprocal is refused on BOTH sides — an agreement, excluded.
    assert!(ri(-f64::MAX, -0.0).powi(-3).is_poison());
    assert!(d_refuses(di(-f64::MAX, -0.0).powi(-3)));
}

/// Re-spelled from the differential's `Ends::chain_spans_zero_and_infinity`.
fn chain_spans_zero_and_infinity(lo: f64, hi: f64, n: i32) -> bool {
    let spans_zero = lo <= 0.0 && hi >= 0.0;
    let mut small = if spans_zero {
        0.0
    } else {
        lo.abs().min(hi.abs())
    };
    let mut large = lo.abs().max(hi.abs());
    let mut reaches_zero = small == 0.0;
    let mut reaches_infinity = large.is_infinite();
    let squarings = (i32::BITS - 1) - (n.unsigned_abs() | 1).leading_zeros();
    for _ in 0..squarings {
        small = (small * small).next_down().max(0.0);
        large = (large * large).next_up();
        reaches_zero |= small == 0.0;
        reaches_infinity |= large.is_infinite();
    }
    reaches_zero && reaches_infinity
}

fn powi_class(lo: f64, hi: f64, n: i32) -> bool {
    n >= 3 && n.count_ones() >= 2 && chain_spans_zero_and_infinity(lo, hi, n)
}

/// The third class's predicate is WIDER than the disagreement it
/// excuses: `[5e-324, 1e300]^3` walks to both an exact 0 (5e-324² rounds
/// to 0) and +inf (1e300² overflows), so the predicate holds — but the
/// ring's actual multiply is `[5e-324, 1e300] · [0, +inf]`, whose four
/// corners are 0, +inf, 0, +inf with no NaN among them, so the ring
/// certifies `[0, +inf]` exactly as the backend does.
#[test]
fn powi_class_predicate_admits_agreements() {
    let (lo, hi) = (5e-324, 1e300);
    assert!(powi_class(lo, hi, 3), "the predicate holds of these inputs");
    let r = ri(lo, hi).powi(3);
    let d = di(lo, hi).powi(3);
    assert!(!r.is_poison(), "but the ring does not poison: {r:?}");
    assert!(!d_refuses(d));
    // Count it over the corner corpus the differential sweeps, at every
    // exponent the lane draws: predicate-holds vs actual disagreement.
    const CORNERS: [f64; 16] = [
        NINF,
        -f64::MAX,
        -1e300,
        -1.0,
        -f64::MIN_POSITIVE,
        -5e-324,
        -0.0,
        0.0,
        5e-324,
        f64::MIN_POSITIVE,
        1e-160,
        1.0,
        1e160,
        1e300,
        f64::MAX,
        INF,
    ];
    let mut holds = 0;
    let mut disagree = 0;
    let mut holds_but_agree = Vec::new();
    for (i, &x) in CORNERS.iter().enumerate() {
        for &y in &CORNERS[i..] {
            let (lo, hi) = if x <= y { (x, y) } else { (y, x) };
            for n in [-3, 0, 1, 2, 3, 5] {
                let p = powi_class(lo, hi, n);
                let dis = ri(lo, hi).powi(n).is_poison() != d_refuses(di(lo, hi).powi(n));
                holds += usize::from(p);
                disagree += usize::from(dis);
                assert!(
                    !dis || p,
                    "a powi disagreement outside the class: [{lo:e},{hi:e}]^{n}"
                );
                if p && !dis {
                    holds_but_agree.push(format!("[{lo:e},{hi:e}]^{n}"));
                }
            }
        }
    }
    println!(
        "powi class over the corner brackets: predicate holds {holds}, disagreements {disagree}, holds-but-agree {}",
        holds_but_agree.len()
    );
    println!("holds-but-agree: {}", holds_but_agree.join(" "));
    assert!(!holds_but_agree.is_empty());
    // Direction: every disagreement in the class is ring-poisons /
    // backend-certifies; the predicate does not say so.
}

// ---------------------------------------------------- division corners

#[test]
fn division_touching_zero_refuses_on_both_sides() {
    let cases = [
        ("straddling", (1.0, 2.0), (-1.0, 1.0)),
        ("exact [0,0]", (1.0, 2.0), (0.0, 0.0)),
        ("[0, +tiny]", (1.0, 2.0), (0.0, 5e-324)),
        ("[-tiny, 0]", (1.0, 2.0), (-5e-324, 0.0)),
        ("[-0.0, 1]", (1.0, 2.0), (-0.0, 1.0)),
        ("zero over zero-touching", (0.0, 0.0), (0.0, 1.0)),
        ("straddling over straddling", (-1.0, 1.0), (-1.0, 1.0)),
        ("unbounded over [0, +inf]", (1.0, INF), (0.0, INF)),
    ];
    for (name, (alo, ahi), (blo, bhi)) in cases {
        let r = ri(alo, ahi) / ri(blo, bhi);
        let d = di(alo, ahi) / di(blo, bhi);
        assert!(r.is_poison(), "{name}: ring should poison");
        assert!(d_refuses(d), "{name}: backend should refuse, got {d:?}");
        if blo == 0.0 && bhi == 0.0 {
            assert!(d.is_empty(), "{name}: the [0,0] divisor is the empty set");
        } else {
            assert_eq!(d.decoration(), Decoration::Trv, "{name}");
            assert!(
                !d.lo().is_nan(),
                "{name}: a Trv bracket carries REAL endpoints: {d:?}"
            );
        }
    }
    // The divisor that is exactly one subnormal away from zero is proven
    // one-signed on both sides: both certify.
    let r = ri(1.0, 2.0) / ri(5e-324, 1.0);
    let d = di(1.0, 2.0) / di(5e-324, 1.0);
    assert!(!r.is_poison() && !d_refuses(d));
    assert!(r.hi().is_infinite() && d.hi().is_infinite());
}

#[cfg(feature = "interval")]
#[test]
fn division_touching_zero_refuses_at_the_interval_scalar() {
    for (blo, bhi) in [
        (-1.0, 1.0),
        (0.0, 0.0),
        (0.0, 5e-324),
        (-5e-324, 0.0),
        (-0.0, 1.0),
    ] {
        assert!(
            i_refuses(1.0, 2.0, |a, b| a / b, blo, bhi),
            "[{blo:e},{bhi:e}]"
        );
    }
    assert!(!i_refuses(1.0, 2.0, |a, b| a / b, 5e-324, 1.0));
}

// --------------------------------------- the collapsed/absent classes

#[test]
fn point_at_infinity_and_inf_minus_inf_and_overflow_agree() {
    assert!(RingInterval::point(INF).is_poison());
    assert!(DInterval::point(INF).is_nai());
    assert!(RingInterval::point(NINF).is_poison() && DInterval::point(NINF).is_nai());
    // Both constructors refuse a closed side at infinity, so inf - inf
    // cannot form: [-inf, 1] - [-inf, 1] is [-inf, +inf] on both.
    let r = ri(NINF, 1.0) - ri(NINF, 1.0);
    let d = di(NINF, 1.0) - di(NINF, 1.0);
    assert!(!r.is_poison() && !d_refuses(d));
    assert_eq!((r.lo(), r.hi()), (NINF, INF));
    assert_eq!((d.lo(), d.hi()), (NINF, INF));
    assert!(ri(INF, INF).is_poison() && di(INF, INF).is_nai());
    // Overflow saturates on both sides and neither refuses.
    let r = ri(1e300, 1e300) * ri(1e300, 1e300);
    let d = di(1e300, 1e300) * di(1e300, 1e300);
    assert!(!r.is_poison() && !d_refuses(d));
    assert!(r.hi().is_infinite() && d.hi().is_infinite());
    assert_eq!(d.decoration(), Decoration::Dac);
}

// ------------------------------------ expected hits at EFFORT 1 (CI)

/// The adversarial regime draws each endpoint uniformly from the 16
/// corner values, so one round is a uniform draw over 16^4 = 65 536
/// (a, b) endpoint quadruples. The expected hits per class per lane at
/// EFFORT 1 is `9 375 adversarial rounds × P(class)`; the other two
/// regimes draw finite values only and cannot form a corner.
#[test]
fn expected_hits_per_class_at_effort_one() {
    const CORNERS: [f64; 16] = [
        NINF,
        -f64::MAX,
        -1e300,
        -1.0,
        -f64::MIN_POSITIVE,
        -5e-324,
        -0.0,
        0.0,
        5e-324,
        f64::MIN_POSITIVE,
        1e-160,
        1.0,
        1e160,
        1e300,
        f64::MAX,
        INF,
    ];
    let mut mul = 0u64;
    let mut div = 0u64;
    let mut powi = 0u64;
    let mut div_zero = 0u64;
    let mut sqr_verdicts = 0u64;
    for &a0 in &CORNERS {
        for &a1 in &CORNERS {
            let (alo, ahi) = if a0 <= a1 { (a0, a1) } else { (a1, a0) };
            for &b0 in &CORNERS {
                for &b1 in &CORNERS {
                    let (blo, bhi) = if b0 <= b1 { (b0, b1) } else { (b1, b0) };
                    let (r, s, d, e) = (ri(alo, ahi), ri(blo, bhi), di(alo, ahi), di(blo, bhi));
                    mul += u64::from((r * s).is_poison() != d_refuses(d * e));
                    let dv = d / e;
                    div += u64::from((r / s).is_poison() != d_refuses(dv));
                    div_zero += u64::from(
                        (r / s).is_poison()
                            && d_refuses(dv)
                            && s.lo() <= 0.0
                            && s.hi() >= 0.0
                            && !s.is_poison(),
                    );
                    sqr_verdicts += 1;
                    for n in [-3, 0, 1, 2, 3, 5] {
                        powi += u64::from(r.powi(n).is_poison() != d_refuses(d.powi(n)));
                    }
                }
            }
        }
    }
    let rounds = 37_500.0 / 4.0;
    let per = |k: u64| rounds * k as f64 / 65_536.0;
    println!(
        "expected hits per lane at EFFORT 1: mul-zero-times-infinite {:.0}, div-infinite-over-infinite {:.0}, powi-chain {:.0}, div-touching-zero agreed {:.0}, sqr verdicts (all regimes) {}",
        per(mul),
        per(div),
        per(powi),
        per(div_zero),
        37_500
    );
    let _ = sqr_verdicts;
    assert!(per(mul) > 100.0 && per(div) > 100.0 && per(powi) > 100.0);
}

// ------------------------------- end-to-end: three dry-run red rows

/// The three consumers, rebuilt by hand at the ring and at the backend
/// the newtype forwards to (`is_poison := dec < Def`).
#[test]
fn three_dry_run_red_rows_by_hand() {
    // (1) certified_door's witness `[0,1] * [0,inf]`: the ring poisons,
    // the newtype certifies `[0, inf]` at Dac — the verdict flips.
    let r = ri(0.0, 1.0) * ri(0.0, INF);
    let d = di(0.0, 1.0) * di(0.0, INF);
    assert!(r.is_poison() && !d_refuses(d));
    // Its sibling `[1,2] / [0,0]` survives: empty is a refusal.
    assert!((ri(1.0, 2.0) / RingInterval::point(0.0)).is_poison());
    assert!(d_refuses(di(1.0, 2.0) / DInterval::point(0.0)));

    // (2) review_m5_pr2_scratch's sign-clamp lane: a product of two
    // same-signed points keeps `lo >= 0` on the ring by the clamp; the
    // backend's underflowed product (3.6e-616 -> 0) has no exactness
    // witness below the 2Prod floor, so it pads to -5e-324. Both
    // certify; the CLAIM (a fact about the reals) is what dies.
    let a = 1.902e-308;
    let r = RingInterval::point(a) * RingInterval::point(a);
    let d = DInterval::point(a) * DInterval::point(a);
    assert_eq!(r.lo(), 0.0);
    assert_eq!(d.lo(), -5e-324);
    assert!(!r.is_poison() && !d_refuses(d));

    // (3) "tighter past an f64 sample": the ring pads every op by one
    // step, so an exact residual `0.5 + 0.25 - 0.75` certifies with a
    // step of width either side of zero; the backend's TwoSum witness
    // proves each step exact and the bound collapses to exactly [0, 0].
    // A sampler whose own rounding is one ulp then lies outside it.
    let r = RingInterval::point(0.5) + RingInterval::point(0.25) - RingInterval::point(0.75);
    let d = DInterval::point(0.5) + DInterval::point(0.25) - DInterval::point(0.75);
    assert!(r.lo() < 0.0 && r.hi() > 0.0, "{r:?}");
    assert_eq!((d.lo(), d.hi()), (0.0, 0.0));
    assert_eq!(d.decoration(), Decoration::Com);
    let sampled = 5e-324; // one representable step of sampler rounding
    assert!(
        r.contains(sampled),
        "the padded ring bound absorbs the sampler's ulp"
    );
    assert!(!d.contains(sampled), "the exact backend bound does not");
}
