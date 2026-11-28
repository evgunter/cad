//! The mint table at each scalar, and the bit-pattern sweep at `f64`
//! and `Interval`. Every expectation is written from the clause
//! (`[sizes]`): the kind by hand, the told ends from the value as
//! built, and the admitted value read back bit for bit. None is
//! computed by the mint's own reads.

use proptest::prelude::*;

use super::*;
use crate::{Dual64, DualInterval, Interval, Sym};

/// What a row expects of the mint: admitted with these ends, or
/// refused with this kind and these told ends.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Expect {
    Admit(f64, f64),
    Zero(f64, f64),
    Negative(f64, f64),
    NaN,
    Unbounded(f64, f64),
}

/// The mint's answer at `x`, in a row's terms. `ends` reads an admitted
/// value's bracket at its own scalar.
fn read<T: Real>(x: T, ends: impl Fn(T) -> (f64, f64)) -> Expect {
    match Size::new(x) {
        Ok(s) => {
            let (lo, hi) = ends(s.get());
            Expect::Admit(lo, hi)
        }
        Err(SizeRefusal::Zero { size }) => {
            let (lo, hi) = size.pinned_ends();
            Expect::Zero(lo, hi)
        }
        Err(SizeRefusal::Negative { size }) => {
            let (lo, hi) = size.pinned_ends();
            Expect::Negative(lo, hi)
        }
        Err(SizeRefusal::NaN) => Expect::NaN,
        Err(SizeRefusal::Unbounded { size }) => {
            let (lo, hi) = size.pinned_ends();
            Expect::Unbounded(lo, hi)
        }
    }
}

/// Two rows' ends agree to the bit, the sign of a zero aside (a told
/// zero is not asked its sign).
fn same(a: Expect, b: Expect) -> bool {
    let eq = |x: f64, y: f64| x == y || (x.is_nan() && y.is_nan());
    match (a, b) {
        (Expect::Admit(a0, a1), Expect::Admit(b0, b1)) => {
            a0.to_bits() == b0.to_bits() && a1.to_bits() == b1.to_bits()
        }
        (Expect::Zero(a0, a1), Expect::Zero(b0, b1))
        | (Expect::Negative(a0, a1), Expect::Negative(b0, b1))
        | (Expect::Unbounded(a0, a1), Expect::Unbounded(b0, b1)) => eq(a0, b0) && eq(a1, b1),
        (Expect::NaN, Expect::NaN) => true,
        _ => false,
    }
}

fn table<T: Real>(scalar: &str, rows: &[(&str, T, Expect)], ends: impl Fn(T) -> (f64, f64) + Copy) {
    for (name, x, want) in rows {
        let got = read(*x, ends);
        assert!(
            same(got, *want),
            "{scalar} {name}: the mint read {got:?}, the clause says {want:?}"
        );
    }
}

fn iv(lo: f64, hi: f64) -> Interval {
    Interval::from_bounds(lo, hi)
}

fn iv_ends(x: Interval) -> (f64, f64) {
    (Bounds::lo(x), Bounds::hi(x))
}

use crate::Bounds;

const MAX: f64 = f64::MAX;
const INF: f64 = f64::INFINITY;
const TINY: f64 = f64::MIN_POSITIVE;
const SUB: f64 = 5e-324;

/// **At `f64`, a size is exactly `0 < x < ∞`**: one row per refusal
/// kind, both signed zeros and both infinities, the smallest subnormal
/// either side of zero, and the admitted edges `MIN_POSITIVE`, the
/// smallest subnormal and `MAX`.
#[test]
fn the_mint_at_f64_admits_exactly_the_finite_positive_values() {
    use Expect::*;
    table(
        "f64",
        &[
            ("0", 0.0, Zero(0.0, 0.0)),
            ("-0", -0.0, Zero(0.0, 0.0)),
            ("-1", -1.0, Negative(-1.0, -1.0)),
            ("-subnormal", -SUB, Negative(-SUB, -SUB)),
            ("-MAX", -MAX, Negative(-MAX, -MAX)),
            ("NaN", f64::NAN, NaN),
            ("+inf", INF, Unbounded(INF, INF)),
            ("-inf", -INF, Unbounded(-INF, -INF)),
            ("subnormal", SUB, Admit(SUB, SUB)),
            ("MIN_POSITIVE", TINY, Admit(TINY, TINY)),
            ("1", 1.0, Admit(1.0, 1.0)),
            ("MAX", MAX, Admit(MAX, MAX)),
        ],
        |x| (x, x),
    );
}

/// **At `Interval`, a size is bounded and not at or below zero at every
/// point**: a straddle is admitted, and so is `[0, b]`; `[0, 0]` and
/// `[−1, 0]` refuse as zero; the poison refuses whether ill-formed or
/// empty; and an end at `∞` refuses even where the width is not poison,
/// `[MAX, ∞]` from an overflow (the row a mint reading
/// `is_finite_length` would pass).
#[test]
fn the_mint_at_interval_admits_a_straddle_and_refuses_an_infinite_end() {
    use Expect::*;
    let empty = iv(-2.0, -1.0).sqrt();
    assert!(
        Real::is_poison(empty) && format!("{empty:?}").contains("Trv"),
        "the empty row is built empty, not ill-formed: {empty:?}"
    );
    let overflow = Interval::from_f64(MAX) * Interval::from_f64(2.0);
    assert!(
        Bounds::lo(overflow) == MAX && Bounds::hi(overflow) == INF,
        "the overflow row is built as [MAX, ∞]: {overflow:?}"
    );
    assert!(
        crate::is_finite_length(overflow),
        "[MAX, ∞] passes is_finite_length, so only this row tells the two finiteness reads apart"
    );
    table(
        "Interval",
        &[
            ("[0, 0]", iv(0.0, 0.0), Zero(0.0, 0.0)),
            ("[-1, 0]", iv(-1.0, 0.0), Zero(-1.0, 0.0)),
            ("[-2, -1]", iv(-2.0, -1.0), Negative(-2.0, -1.0)),
            ("[-1, -subnormal]", iv(-1.0, -SUB), Negative(-1.0, -SUB)),
            ("NaI", Interval::from_f64(f64::NAN), NaN),
            ("empty", empty, NaN),
            ("[MAX, inf] by overflow", overflow, Unbounded(MAX, INF)),
            ("[MAX, inf]", iv(MAX, INF), Unbounded(MAX, INF)),
            ("[-inf, -1]", iv(-INF, -1.0), Unbounded(-INF, -1.0)),
            ("[-inf, 1]", iv(-INF, 1.0), Unbounded(-INF, 1.0)),
            ("entire", iv(-INF, INF), Unbounded(-INF, INF)),
            ("[-1, 1]", iv(-1.0, 1.0), Admit(-1.0, 1.0)),
            ("[-1, subnormal]", iv(-1.0, SUB), Admit(-1.0, SUB)),
            ("[0, 1]", iv(0.0, 1.0), Admit(0.0, 1.0)),
            ("[MIN_POSITIVE]", iv(TINY, TINY), Admit(TINY, TINY)),
            ("[MAX]", iv(MAX, MAX), Admit(MAX, MAX)),
        ],
        iv_ends,
    );
}

/// **At the dual scalars the value channel decides**, as it does every
/// exact read: the `f64` and `Interval` tables' edges again, and a
/// finite value with a poisoned tangent is a size (the tangent is not
/// a length).
#[test]
fn the_mint_at_dual_reads_the_value_channel() {
    use Expect::*;
    let d = |v: f64| Dual64::new(v, 1.0);
    table(
        "Dual64",
        &[
            ("0", d(0.0), Zero(0.0, 0.0)),
            ("-1", d(-1.0), Negative(-1.0, -1.0)),
            ("NaN", d(f64::NAN), NaN),
            ("+inf", d(INF), Unbounded(INF, INF)),
            ("MIN_POSITIVE", d(TINY), Admit(TINY, TINY)),
            ("MAX", d(MAX), Admit(MAX, MAX)),
            (
                "1 with a NaN tangent",
                Dual64::new(1.0, f64::NAN),
                Admit(1.0, 1.0),
            ),
        ],
        |x| (x.value, x.value),
    );
    let di = |lo: f64, hi: f64| DualInterval::new(iv(lo, hi), Interval::from_f64(1.0));
    table(
        "DualInterval",
        &[
            ("[-1, 0]", di(-1.0, 0.0), Zero(-1.0, 0.0)),
            ("[-2, -1]", di(-2.0, -1.0), Negative(-2.0, -1.0)),
            (
                "NaI",
                DualInterval::new(Interval::from_f64(f64::NAN), Interval::from_f64(1.0)),
                NaN,
            ),
            ("[MAX, inf]", di(MAX, INF), Unbounded(MAX, INF)),
            ("[-1, 1]", di(-1.0, 1.0), Admit(-1.0, 1.0)),
            ("[MAX]", di(MAX, MAX), Admit(MAX, MAX)),
        ],
        |x| iv_ends(x.value),
    );
}

/// **At the symbolic scalar the numeric channel decides**: the same
/// edges over `f64` and over `Interval`, each value held as an opaque
/// leaf so no rule of the symbolic tier reads it.
#[test]
fn the_mint_at_sym_reads_the_numeric_channel() {
    use Expect::*;
    table(
        "Sym<f64>",
        &[
            ("0", Sym::opaque(0.0), Zero(0.0, 0.0)),
            ("-1", Sym::opaque(-1.0), Negative(-1.0, -1.0)),
            ("NaN", Sym::opaque(f64::NAN), NaN),
            ("+inf", Sym::opaque(INF), Unbounded(INF, INF)),
            ("MIN_POSITIVE", Sym::opaque(TINY), Admit(TINY, TINY)),
            ("MAX", Sym::opaque(MAX), Admit(MAX, MAX)),
        ],
        |x: Sym<f64>| (Bounds::lo(x), Bounds::hi(x)),
    );
    let s = |lo: f64, hi: f64| Sym::opaque(iv(lo, hi));
    table(
        "Sym<Interval>",
        &[
            ("[-1, 0]", s(-1.0, 0.0), Zero(-1.0, 0.0)),
            ("[-2, -1]", s(-2.0, -1.0), Negative(-2.0, -1.0)),
            ("NaI", Sym::opaque(Interval::from_f64(f64::NAN)), NaN),
            ("[MAX, inf]", s(MAX, INF), Unbounded(MAX, INF)),
            ("[-1, 1]", s(-1.0, 1.0), Admit(-1.0, 1.0)),
            ("[MAX]", s(MAX, MAX), Admit(MAX, MAX)),
        ],
        |x: Sym<Interval>| (Bounds::lo(x), Bounds::hi(x)),
    );
}

/// **The constant is minted, and is one**, at every scalar.
#[test]
fn the_unit_size_is_one_at_every_scalar() {
    assert_eq!(Size::<f64>::one().get().to_bits(), 1.0_f64.to_bits(), "f64");
    assert_eq!(
        iv_ends(Size::<Interval>::one().get()),
        (1.0, 1.0),
        "Interval"
    );
    assert_eq!(
        Size::<Dual64>::one().get().value.to_bits(),
        1.0_f64.to_bits(),
        "Dual64"
    );
    assert_eq!(
        iv_ends(Size::<DualInterval>::one().get().value),
        (1.0, 1.0),
        "DualInterval"
    );
    assert_eq!(Bounds::lo(Size::<Sym<f64>>::one().get()), 1.0, "Sym<f64>");
    assert_eq!(
        Bounds::hi(Size::<Sym<Interval>>::one().get()),
        1.0,
        "Sym<Interval>"
    );
}

/// The refusal names its kind and tells the value.
#[test]
fn a_refusal_says_its_kind_and_its_value() {
    let text = |x: f64| Size::new(x).unwrap_err().to_string();
    assert_eq!(text(0.0), "size 0 m is zero");
    assert_eq!(text(-2.5), "size -2.5 m is negative");
    assert_eq!(text(f64::NAN), "size is not a number");
    assert!(
        text(INF).ends_with("is not a finite length"),
        "{}",
        text(INF)
    );
}

/// The clause's kind at `f64`, from IEEE comparisons alone.
fn f64_oracle(x: f64) -> Expect {
    if x.is_nan() {
        Expect::NaN
    } else if x.is_infinite() {
        Expect::Unbounded(x, x)
    } else if x == 0.0 {
        Expect::Zero(0.0, 0.0)
    } else if x < 0.0 {
        Expect::Negative(x, x)
    } else {
        Expect::Admit(x, x)
    }
}

/// The clause's kind at `Interval`, from the ends alone, for ends that
/// make a nonempty enclosure.
fn interval_oracle(lo: f64, hi: f64) -> Expect {
    if !(lo.is_finite() && hi.is_finite()) {
        Expect::Unbounded(lo, hi)
    } else if hi < 0.0 {
        Expect::Negative(lo, hi)
    } else if hi == 0.0 {
        Expect::Zero(lo, 0.0)
    } else {
        Expect::Admit(lo, hi)
    }
}

/// Bit patterns weighted toward the edges the clause names.
fn edgy_f64() -> impl Strategy<Value = f64> {
    prop_oneof![
        any::<u64>().prop_map(f64::from_bits),
        prop::sample::select(vec![
            0.0,
            -0.0,
            SUB,
            -SUB,
            TINY,
            -TINY,
            MAX,
            -MAX,
            INF,
            -INF,
            f64::NAN,
            1.0,
            -1.0
        ]),
    ]
}

proptest! {
    /// **The mint never panics at `f64`, and admits exactly
    /// `0 < x < ∞`**, on any bit pattern (counterexample search).
    #[test]
    fn the_mint_at_f64_matches_the_clause_on_every_bit_pattern(x in edgy_f64()) {
        let got = read(x, |v| (v, v));
        prop_assert!(same(got, f64_oracle(x)), "x = {x:?}: {got:?}");
    }

    /// **The mint never panics at `Interval`, and admits exactly the
    /// bounded enclosures not at or below zero at every point**, on any
    /// pair of bit patterns: where the pair is no enclosure it builds
    /// the poison, which refuses as NaN.
    #[test]
    fn the_mint_at_interval_matches_the_clause_on_every_pair(lo in edgy_f64(), hi in edgy_f64()) {
        let got = read(iv(lo, hi), iv_ends);
        let encloses = !lo.is_nan() && !hi.is_nan() && lo <= hi && lo != INF && hi != -INF;
        let want = if encloses { interval_oracle(lo, hi) } else { Expect::NaN };
        prop_assert!(same(got, want), "[{lo:?}, {hi:?}]: {got:?}, want {want:?}");
    }
}
