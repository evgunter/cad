//! **Hand-built rows beside the ring differential.**
//!
//! `ring_interval_differential.rs` sweeps: its corner corpus is
//! exhaustive and its fuzz lanes hunt for an operation that stopped
//! forwarding. What it does not do is write down, in one readable
//! place, the rows a reader has to take on trust — the sentences its
//! module doc states about shapes that look like disagreements and are
//! not, the negative-exponent chain that is the ring's second producer
//! of a refusal carrying real endpoints, the division corners the H5
//! survey asked about, and the characterised corners where the ring's
//! answer MOVED when its arithmetic became the backend's. Each of
//! those is a row here, asserted rather than printed.
//!
//! The corners in the last group are the ones the retired arithmetic
//! answered differently: an unconditional outward ulp per operation, a
//! sign clamp, a zero annihilator and a NaN-propagating corner
//! reduction. Nothing here compares the two — there is one arithmetic
//! now — so each row states what the tree answers, which is what a
//! consumer reading a certificate needs.

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

/// The backend's refusal, as the differential spells it — and, read
/// through the ring's own accessor, as `RingInterval::is_poison`.
fn d_refuses(d: DInterval) -> bool {
    d.is_nai() || d.is_empty() || d.decoration() < Decoration::Def
}

// ---------------------------------------------------- division corners

/// The division claim, in the shape the H5 survey asked it and the
/// shape that is true: **a divisor that touches or straddles zero is
/// refused**, and the refusal carries REAL endpoints (`Trv`) except
/// for the exact `[0,0]` divisor, which is the empty set.
///
/// That is the whole reason a consumer reading one endpoint of a
/// quotient has to ask `is_poison()` rather than compare against NaN.
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
        assert!(!r.is_certified(), "{name}: the quotient should refuse");
        assert!(d_refuses(d), "{name}: backend should refuse, got {d:?}");
        if blo == 0.0 && bhi == 0.0 {
            assert!(d.is_empty(), "{name}: the [0,0] divisor is the empty set");
        } else {
            assert_eq!(d.decoration(), Decoration::Trv, "{name}");
            assert!(
                !d.lo().is_nan() && !r.lo().is_nan(),
                "{name}: a Trv bracket carries REAL endpoints: {d:?}"
            );
        }
    }
    // The divisor that is exactly one subnormal away from zero is
    // proven one-signed, and certifies.
    let r = ri(1.0, 2.0) / ri(5e-324, 1.0);
    assert!(r.is_certified() && r.hi().is_infinite());
    // Unbounded over unbounded is NOT a refusal: the `inf / inf` corner
    // is dropped by the backend's min/max fold, which leaves a sound
    // bracket built from the remaining corners at `Dac`.
    let r = ri(1.0, INF) / ri(1.0, INF);
    let d = di(1.0, INF) / di(1.0, INF);
    assert!(r.is_certified() && !d_refuses(d), "{d:?}");
    assert_eq!(d.decoration(), Decoration::Dac);
}

#[cfg(feature = "interval")]
#[test]
fn division_touching_zero_refuses_at_the_interval_scalar() {
    // The scalar wrapper's refusal, spelled inline rather than through
    // a gated helper: `scripts/check-interval-cfg-additive.py` admits
    // only whole gated items of a few kinds under `crates/*/tests`, so
    // that a name present in both builds runs the same code in both.
    let refuses = |blo: f64, bhi: f64| {
        let a = geom_core::Interval::from_bounds(1.0, 2.0);
        let b = geom_core::Interval::from_bounds(blo, bhi);
        !(a / b).is_certified()
    };
    for (blo, bhi) in [
        (-1.0, 1.0),
        (0.0, 0.0),
        (0.0, 5e-324),
        (-5e-324, 0.0),
        (-0.0, 1.0),
    ] {
        assert!(refuses(blo, bhi), "[{blo:e},{bhi:e}]");
    }
    assert!(!refuses(5e-324, 1.0));
}

// --------------------------------------- the shapes that are not there

/// The differential's module doc names two shapes a reader expects to
/// be disagreements and will not find. This is those two sentences,
/// executed.
#[test]
fn point_at_infinity_and_inf_minus_inf_are_agreements() {
    assert!(!RingInterval::point(INF).is_certified());
    assert!(DInterval::point(INF).is_nai());
    assert!(!RingInterval::point(NINF).is_certified() && DInterval::point(NINF).is_nai());
    // Both constructors refuse a closed side at infinity, so inf - inf
    // cannot form: [-inf, 1] - [-inf, 1] is [-inf, +inf].
    let r = ri(NINF, 1.0) - ri(NINF, 1.0);
    let d = di(NINF, 1.0) - di(NINF, 1.0);
    assert!(r.is_certified() && !d_refuses(d));
    assert_eq!((r.lo(), r.hi()), (NINF, INF));
    assert_eq!((d.lo(), d.hi()), (NINF, INF));
    assert!(!ri(INF, INF).is_certified() && di(INF, INF).is_nai());
}

/// Overflow saturates to `±inf` and is honest about the unbounded
/// side — under `+ − × ÷` and under `powi` alike. It is not a refusal
/// anywhere: an overflowed bound is a bound.
#[test]
fn overflow_agrees_on_every_ring_op_including_powi() {
    let r = ri(1e300, 1e300) * ri(1e300, 1e300);
    let d = di(1e300, 1e300) * di(1e300, 1e300);
    assert!(r.is_certified() && !d_refuses(d));
    assert!(r.hi().is_infinite() && d.hi().is_infinite());
    assert_eq!(d.decoration(), Decoration::Dac);
    // An overflow INSIDE a `powi` chain pairs a zero endpoint with an
    // infinite one at a multiply. A corner reduction that propagated
    // the resulting NaN would poison here; the backend's `pow_pos`
    // never forms the product, and the power is the honest unbounded
    // bracket.
    let r = ri(-f64::MAX, -0.0).powi(3);
    let d = di(-f64::MAX, -0.0).powi(3);
    assert!(r.is_certified() && !d_refuses(d), "{d:?}");
    assert_eq!((r.lo(), r.hi()), (NINF, 0.0));
}

/// **A negative power is a division**, and it is the ring's second
/// producer of a refusal carrying real endpoints — the first being an
/// ordinary division, and the only other way in being a crossing from
/// an uncertified scalar.
///
/// `pow_pos` starts its accumulator at `[1, 1]` and multiplies, and
/// below the 2Prod floor that multiply has no exactness witness, so it
/// pads: the first power of `[5e-324, 1]` comes out as `[0, 1]` and its
/// reciprocal divides by a zero-touching bracket. Dividing by the same
/// base by hand certifies, which is what makes this a chain fact rather
/// than a division one — and what makes the exponent, not the base, the
/// thing to read at a call site.
///
/// **No production call site reaches it**: every ring exponent in
/// `crates/*/src` is positive (`2`…`5`, in `props/quad.rs`'s quadrature
/// weights and `offset_fit.rs`'s `w.powi(3)`).
#[test]
fn a_negative_power_can_refuse_where_the_division_by_hand_does_not() {
    let a = (5e-324, 1.0);
    assert_eq!(
        (ri(a.0, a.1).powi(1).lo(), ri(a.0, a.1).powi(1).hi()),
        (0.0, 1.0)
    );

    let r = ri(a.0, a.1).powi(-1);
    let d = di(a.0, a.1).powi(-1);
    assert!(!r.is_certified() && d_refuses(d));
    assert_eq!(d.decoration(), Decoration::Trv);
    // And it carries real endpoints, which is the hazard shape.
    assert!(!r.lo().is_nan() && !r.hi().is_nan(), "{r:?}");

    // The same reciprocal spelled as a division certifies.
    let by_hand = RingInterval::point(1.0) / ri(a.0, a.1);
    assert!(by_hand.is_certified(), "{by_hand:?}");
    assert!(!d_refuses(DInterval::point(1.0) / di(a.0, a.1)));

    // One squaring in, the same story: a base whose square lands in the
    // deep subnormal range refuses at `n = -2`.
    let b = (3.045_808_901_121_822e-162, 2.994_982_710_385_136_5e-31);
    assert!(!ri(b.0, b.1).powi(-2).is_certified());
}

// ------------------------- the corners where the ring's answer moved

/// The characterised corners the RING-2 swap moved, each stated as
/// what the tree answers now. These are the consumers' rows: a test
/// asserting any of the retired answers is asserting a rule the
/// arithmetic no longer has.
#[test]
fn the_characterised_corners_the_swap_moved() {
    // (1) `[0,1] * [0,inf]`: the `0 · inf` corner is resolved, not
    // propagated — `mul_lo`/`mul_hi` answer `0` for a zero operand and
    // the min/max fold drops nothing. The product certifies.
    let r = ri(0.0, 1.0) * ri(0.0, INF);
    assert!(r.is_certified() && (r.lo(), r.hi()) == (0.0, INF), "{r:?}");
    // `[1,2] / [0,0]` still refuses: it is the empty set.
    assert!(!(ri(1.0, 2.0) / RingInterval::point(0.0)).is_certified());

    // (2) THE SIGN CLAMP IS GONE. A product of two opposite-signed
    // subnormals has no exactness witness below the 2Prod floor, so the
    // outward pad stands: `hi` is `5e-324` where the clamp gave exactly
    // `0`. Both endpoints are sound bounds of the true product; what
    // died is the CLAIM that a provably-nonpositive product has
    // `hi <= 0`, which was a fact about the reals imposed on top of the
    // arithmetic rather than one the arithmetic proves.
    let (a, b) = (2.2250738585072014e-308, -1.8669573922462645e-308);
    let p = RingInterval::point(a) * RingInterval::point(b);
    assert_eq!(p.hi(), 5e-324);
    assert!(p.is_certified());
    // The same-sign `lo` arm moves the same way.
    let c = 1.902e-308;
    assert_eq!(
        (RingInterval::point(c) * RingInterval::point(c)).lo(),
        -5e-324
    );

    // (3) THE EXACTNESS WITNESSES FIRE. An exact residual
    // `0.5 + 0.25 - 0.75` collapses to exactly `[0, 0]` rather than
    // carrying a pad either side — which is why a test that sampled the
    // same expression in `f64` and demanded the certified bound contain
    // its own rounding error stopped holding.
    let e = RingInterval::point(0.5) + RingInterval::point(0.25) - RingInterval::point(0.75);
    assert_eq!((e.lo(), e.hi()), (0.0, 0.0));
    assert!(!e.contains(5e-324), "the exact bound admits no slack");
    assert_eq!(
        di(0.5, 0.5).powi(1).decoration(),
        Decoration::Com,
        "a bounded exact bracket keeps Com"
    );

    // (4) The zero annihilator is not a rule any more, and it does not
    // need to be: the backend resolves the same corner.
    let r = ri(0.0, 0.0) * ri(NINF, INF);
    assert!(r.is_certified() && (r.lo(), r.hi()) == (0.0, 0.0));

    // (5) Production-shaped weight hulls under `powi(3)`, the shape
    // `props/quad.rs` runs: every one of them certifies, including the
    // one whose chain reaches both zero and infinity.
    for (lo, hi) in [(0.5, 2.0), (0.0, 1.0), (1e-200, 1e200), (0.0, INF)] {
        assert!(
            ri(lo, hi).powi(3).is_certified(),
            "[{lo:e}, {hi:e}].powi(3) refused"
        );
    }
}
