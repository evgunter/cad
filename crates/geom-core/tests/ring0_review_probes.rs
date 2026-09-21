//! **Hand-built rows beside the ring differential.**
//!
//! `ring_interval_differential.rs` sweeps: its corner corpus is
//! exhaustive over the allowlist's classes and its fuzz lanes hunt for
//! a fifth. What it does not do is write down, in one readable place,
//! the rows a reader has to take on trust — the sentences its module
//! doc states about shapes that are NOT classes, the mechanism behind
//! the one class that runs backend-refuses, the division corners the
//! H5 survey asked about, and the consumers the RING-2 newtype dry run
//! turned red. Each of those is a row here, asserted rather than
//! printed.
//!
//! Nothing here re-tests the differential's own closure claim: a
//! disagreement outside the allowlist fails there, at every draw.

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

// ---------------------------------------------------- division corners

/// The division claim, in the shape the H5 survey asked it and the
/// shape that is true: **a divisor that touches or straddles zero is
/// refused by both arithmetics** — which is not "zero disagreements on
/// division", because unbounded-over-unbounded is a class.
///
/// The backend's refusal here carries REAL endpoints (`Trv`), which is
/// the whole reason the newtype swap is a verdict question and not an
/// endpoint one.
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
    // And unbounded-over-unbounded is the one division class: the ring
    // poisons on the `inf / inf` corner where the backend does not.
    let r = ri(1.0, INF) / ri(1.0, INF);
    let d = di(1.0, INF) / di(1.0, INF);
    assert!(r.is_poison() && !d_refuses(d), "{d:?}");
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

/// The differential's module doc names two shapes a reader expects to
/// be classes and will not find. This is those two sentences, executed.
#[test]
fn point_at_infinity_and_inf_minus_inf_are_agreements() {
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
}

/// Overflow is an agreement for `+ − × ÷` and is NOT one for `powi`:
/// the third sentence of that list used to say it was a non-class
/// outright, which the negative exponents refute.
#[test]
fn overflow_agrees_on_the_ring_ops_and_not_on_powi() {
    // The product saturates to +inf on both sides and neither refuses.
    let r = ri(1e300, 1e300) * ri(1e300, 1e300);
    let d = di(1e300, 1e300) * di(1e300, 1e300);
    assert!(!r.is_poison() && !d_refuses(d));
    assert!(r.hi().is_infinite() && d.hi().is_infinite());
    assert_eq!(d.decoration(), Decoration::Dac);
    // Under `powi` the two chains part company: an overflow inside the
    // chain supplies the `inf` half of the multiply corner the ring
    // poisons on, where the backend certifies.
    let r = ri(-f64::MAX, -0.0).powi(3);
    let d = di(-f64::MAX, -0.0).powi(3);
    assert!(r.is_poison() && !d_refuses(d), "{d:?}");
}

/// The mechanism behind the one class that runs the other way, pinned
/// where the differential's predicate only asserts its consequence.
///
/// The backend's `pow_pos` starts its accumulator at `[1, 1]` and
/// multiplies, and below the 2Prod floor that multiply has no exactness
/// witness, so it pads: the positive power of `[5e-324, 1]` comes out
/// as `[0, 1]` and its reciprocal divides by a zero-touching bracket.
/// The ring seeds the accumulator at the first set bit instead, so its
/// `powi(1)` is the base untouched and its reciprocal certifies. Divide
/// by the same base by hand and the backend certifies too, which is
/// what makes this a chain fact rather than a division one.
#[test]
fn the_negative_exponent_class_is_the_backends_extra_pad() {
    let a = (5e-324, 1.0);
    assert_eq!(
        (di(a.0, a.1).powi(1).lo(), di(a.0, a.1).powi(1).hi()),
        (0.0, 1.0)
    );
    assert_eq!((ri(a.0, a.1).powi(1).lo(), ri(a.0, a.1).powi(1).hi()), a);

    let d = di(a.0, a.1).powi(-1);
    let r = ri(a.0, a.1).powi(-1);
    assert!(d_refuses(d) && !r.is_poison());
    assert_eq!(d.decoration(), Decoration::Trv);
    assert!(r.hi().is_infinite() && r.lo() < 1.0);

    // The same reciprocal spelled as a division certifies on both.
    let by_hand = DInterval::point(1.0) / di(a.0, a.1);
    assert!(!d_refuses(by_hand), "{by_hand:?}");
    assert!(!(RingInterval::point(1.0) / ri(a.0, a.1)).is_poison());

    // And the class survives one squaring: a base whose square lands in
    // the deep subnormal range is the same story at `n = -2`.
    let b = (3.045_808_901_121_822e-162, 2.994_982_710_385_136_5e-31);
    assert!(di(b.0, b.1).powi(-2).decoration() < Decoration::Def);
    assert!(!ri(b.0, b.1).powi(-2).is_poison());
}

// ------------------------------------ expected hits at EFFORT 1 (CI)

/// The adversarial regime draws each endpoint uniformly from the 16
/// corner values, so one round is a uniform draw over 16^4 = 65 536
/// (a, b) endpoint quadruples. The expected hits per class per lane at
/// `CAD_FUZZ_EFFORT=1` is `9 375 adversarial rounds × P(class)`; the
/// other two regimes draw finite values only and cannot form a corner.
///
/// The assertion is that the shallowest CI run still reaches every
/// class by a wide margin — the fuzz lanes cannot go green by
/// starvation.
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
    const EXPONENTS: [i32; 11] = [-3, -2, -1, 0, 1, 2, 3, 5, 6, 7, 31];
    let mut mul = 0u64;
    let mut div = 0u64;
    let mut powi_pos = 0u64;
    let mut powi_neg = 0u64;
    let mut div_zero = 0u64;
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
                    for n in EXPONENTS {
                        let hit = u64::from(r.powi(n).is_poison() != d_refuses(d.powi(n)));
                        if n < 0 {
                            powi_neg += hit;
                        } else {
                            powi_pos += hit;
                        }
                    }
                }
            }
        }
    }
    let rounds = 37_500.0 / 4.0;
    let per = |k: u64| rounds * k as f64 / 65_536.0;
    println!(
        "expected hits per lane at EFFORT 1: mul-zero-times-infinite {:.0}, \
         div-infinite-over-infinite {:.0}, powi-chain-corner {:.0}, \
         powi-negative-reciprocal {:.0}, div-touching-zero agreed {:.0}",
        per(mul),
        per(div),
        per(powi_pos),
        per(powi_neg),
        per(div_zero)
    );
    assert!(per(mul) > 100.0 && per(div) > 100.0);
    assert!(per(powi_pos) > 100.0 && per(powi_neg) > 100.0);
}

// ------------------------------- end-to-end: the dry run's red rows

/// The consumers the newtype dry run turned red, rebuilt by hand at the
/// ring and at the backend the newtype forwards to
/// (`is_poison := dec < Def`). The dry-run branch never merges, so this
/// is where its evidence survives.
#[test]
fn dry_run_red_rows_by_hand() {
    // (1) certified_door's witness `[0,1] * [0,inf]`: the ring poisons,
    // the newtype certifies `[0, inf]` at Dac — the verdict flips.
    let r = ri(0.0, 1.0) * ri(0.0, INF);
    let d = di(0.0, 1.0) * di(0.0, INF);
    assert!(r.is_poison() && !d_refuses(d));
    // Its sibling `[1,2] / [0,0]` survives: empty is a refusal.
    assert!((ri(1.0, 2.0) / RingInterval::point(0.0)).is_poison());
    assert!(d_refuses(di(1.0, 2.0) / DInterval::point(0.0)));

    // (2) review_m5_pr2_scratch's sign-clamp lane: the ring's clamp
    // keeps a provably-signed product's endpoint at exactly zero where
    // the backend's underflowed product has no exactness witness below
    // the 2Prod floor and pads past it. Both certify; the CLAIM (a
    // fact about the reals) is what dies. The failing arm is the
    // OPPOSITE-sign `hi`: a positive times a negative subnormal.
    let (a, b) = (2.2250738585072014e-308, -1.8669573922462645e-308);
    let r = RingInterval::point(a) * RingInterval::point(b);
    let d = DInterval::point(a) * DInterval::point(b);
    assert_eq!(r.hi(), 0.0);
    assert_eq!(d.hi(), 5e-324);
    assert!(!r.is_poison() && !d_refuses(d));
    // The same-sign `lo` arm moves the same way.
    let c = 1.902e-308;
    assert_eq!((RingInterval::point(c) * RingInterval::point(c)).lo(), 0.0);
    assert_eq!((DInterval::point(c) * DInterval::point(c)).lo(), -5e-324);

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

    // (4) the zero annihilator, which the newtype drops: the ring
    // answers `[0,0]` for a zero factor against anything, the backend
    // resolves the same corner as `[0, 0]` at Dac rather than poison.
    let r = ri(0.0, 0.0) * ri(NINF, INF);
    let d = di(0.0, 0.0) * di(NINF, INF);
    assert!(!r.is_poison() && (r.lo(), r.hi()) == (0.0, 0.0));
    assert!(!d_refuses(d) && (d.lo(), d.hi()) == (0.0, 0.0));

    // (5) a production-shaped weight hull under `powi(3)`: the shape
    // `props/quad.rs` runs. A bounded, one-signed hull agrees, and so
    // does a hull whose chain reaches BOTH zero and infinity without
    // pairing them in one multiply — `[1e-200, 1e200]` squares to
    // `[0, inf]` and the multiply's four corners are 0, inf, 0, inf
    // with no NaN among them. Only an operand pair that puts the zero
    // on one side and the infinity on the other is class 3.
    assert!(!ri(0.5, 2.0).powi(3).is_poison() && !d_refuses(di(0.5, 2.0).powi(3)));
    assert!(!ri(0.0, 1.0).powi(3).is_poison() && !d_refuses(di(0.0, 1.0).powi(3)));
    assert!(!ri(1e-200, 1e200).powi(3).is_poison());
    assert!(ri(0.0, INF).powi(3).is_poison() && !d_refuses(di(0.0, INF).powi(3)));
}
