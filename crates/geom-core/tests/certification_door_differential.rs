//! **Differential lane: the certification doors that are more than a
//! delegate, each against its contract spelled over the backend.**
//!
//! `Certification::hull`, `clamped_to`, `contains`, `width` and `mag`
//! each add a refusal or a reading of their own on top of
//! [`DInterval`], so a forwarding comparison cannot see them. Here each
//! door is compared with a reference written from its doc over the
//! backend twin's endpoints and decoration — `width`'s over exact
//! integer arithmetic, since its rounding is the point — and no
//! reference calls a door or the backend operation the door is built on.
//!
//! Per case the refusal verdict is asserted first (for a scalar reading,
//! "is NaN"), then the answer: both endpoints and the decoration bit for
//! bit through [`Interval::repr_bits`], a refusal's shape as NaI, a
//! reading's bits.
//!
//! **One carve-out, and why.** Where an endpoint is chosen between two
//! zeros of opposite sign (`hull([-0, 1], [0, 1])`, a clamp of `[-0, 1]`
//! to `[0, 1]`), the reference compares the zero by value: neither door
//! states a sign, and `f64::min`/`max`, which both are built on, do not
//! fix one for zeros of opposite sign (on x86-64 the second operand's
//! wins, so `hull` is not bit-commutative there). Every other endpoint
//! is compared by its bits.
//!
//! The corpus is an enumeration: every bracket over
//! `interval_backend_differential`'s [`CORNERS`] plus [`EXTRAS`], each at
//! its own decoration, capped at `Def`, and at `Trv` with its real
//! endpoints kept; the empty set; and, for `clamped_to` and `contains`,
//! every window bound and probe one ulp either side of the operand's
//! endpoints.

use geom_core::Interval;
use geom_core::interval::certification::Certification;
use interval_transcendentals::{DInterval, Decoration};
use num_bigint::{BigInt, BigUint, Sign};

use crate::interval_backend_differential::CORNERS;

/// Values the doors' contracts turn on that [`CORNERS`] lacks: brackets
/// whose width is inexact (`[0.1, 0.3]`), and two whose exact width is
/// a tie between neighbouring doubles, one rounding down to even and one
/// up.
const EXTRAS: [f64; 4] = [
    0.1,
    0.3,
    -1.110_223_024_625_156_5e-16,
    1.000_000_000_000_000_2,
];

/// One operand: the scalar and its backend twin, built by the same
/// recipe and checked to be the same stored value.
#[derive(Clone, Copy)]
struct Operand {
    ring: Interval,
    back: DInterval,
}

impl core::fmt::Display for Operand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:e}, {:e}]@{:?}",
            self.back.lo(),
            self.back.hi(),
            self.back.decoration()
        )
    }
}

fn dec_code(d: Decoration) -> u8 {
    match d {
        Decoration::Ill => 0,
        Decoration::Trv => 1,
        Decoration::Def => 2,
        Decoration::Dac => 3,
        Decoration::Com => 4,
    }
}

fn operand(ring: Interval, back: DInterval) -> Operand {
    let twin = (
        back.lo().to_bits(),
        back.hi().to_bits(),
        dec_code(back.decoration()),
    );
    assert_eq!(
        ring.repr_bits(),
        twin,
        "corpus: the scalar {ring:?} and its backend twin {back:?} are not the same value"
    );
    Operand { ring, back }
}

fn corpus() -> Vec<Operand> {
    let values: Vec<f64> = CORNERS.iter().chain(&EXTRAS).copied().collect();
    // A refused zero with real endpoints: adding it keeps a bracket's
    // endpoints and drops its decoration to `Trv`.
    let trv_zero = operand(
        (Interval::from_bounds(-2.0, -1.0) / Interval::from_bounds(-1.0, 1.0)) * Interval::zero(),
        (DInterval::from_bounds(-2.0, -1.0) / DInterval::from_bounds(-1.0, 1.0))
            * DInterval::point(0.0),
    );
    let mut out = Vec::new();
    for (i, &x) in values.iter().enumerate() {
        for &y in &values[i..] {
            let (lo, hi) = if x <= y { (x, y) } else { (y, x) };
            let ring = Interval::from_bounds(lo, hi);
            let back = DInterval::from_bounds(lo, hi);
            out.push(operand(ring, back));
            out.push(operand(
                Interval::from_certified(ring),
                back.with_dec_capped(Decoration::Def),
            ));
            out.push(operand(ring + trv_zero.ring, back + trv_zero.back));
        }
    }
    out.push(operand(
        Interval::one() / Interval::zero(),
        DInterval::point(1.0) / DInterval::point(0.0),
    ));
    out
}

/// The backend's refusal, read off the twin: below `Def`, which takes in
/// NaI (`Ill`) and the empty set (`Trv`).
fn refuses(d: DInterval) -> bool {
    d.decoration() < Decoration::Def
}

/// The window bounds and membership probes worth trying against `x`:
/// its endpoints and one ulp either side of each, plus fixed edges.
fn probes(x: DInterval) -> Vec<f64> {
    let mut out = vec![
        f64::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        1.0,
        f64::INFINITY,
        f64::NAN,
    ];
    for e in [x.lo(), x.hi()] {
        if !e.is_nan() {
            out.extend([e.next_down(), e, e.next_up()]);
        }
    }
    out
}

// ------------------------------------------------ interval-valued doors

/// An expected endpoint. `zero_sign_open` marks the one case the doors
/// leave unspecified: a choice between two zeros of opposite sign.
#[derive(Clone, Copy)]
struct End {
    v: f64,
    zero_sign_open: bool,
}

fn chosen(a: f64, b: f64, want_lesser: bool) -> End {
    let v = if a < b {
        if want_lesser { a } else { b }
    } else if b < a {
        if want_lesser { b } else { a }
    } else {
        a
    };
    End {
        v,
        zero_sign_open: a.to_bits() != b.to_bits(),
    }
}

/// A door's expected bracket; `None` is NaI.
struct Want {
    lo: End,
    hi: End,
    dec: Decoration,
}

/// `hull`: NaI when either operand refuses — the empty set included,
/// which the backend's own hull would absorb; otherwise the least
/// bracket over both, at the weaker of their decorations.
fn hull_reference(a: DInterval, b: DInterval) -> Option<Want> {
    if refuses(a) || refuses(b) {
        return None;
    }
    Some(Want {
        lo: chosen(a.lo(), b.lo(), true),
        hi: chosen(a.hi(), b.hi(), false),
        dec: a.decoration().min(b.decoration()),
    })
}

/// `clamped_to`: NaI for a refused enclosure, a NaN window bound, or an
/// intersection holding no real number; otherwise the intersection, at
/// the enclosure's own decoration.
fn clamp_reference(x: DInterval, lo: f64, hi: f64) -> Option<Want> {
    if refuses(x) || lo.is_nan() || hi.is_nan() {
        return None;
    }
    let lo = chosen(x.lo(), lo, false);
    let hi = chosen(x.hi(), hi, true);
    if lo.v > hi.v || lo.v == f64::INFINITY || hi.v == f64::NEG_INFINITY {
        return None;
    }
    Some(Want {
        lo,
        hi,
        dec: x.decoration(),
    })
}

fn same_end(got: f64, want: End) -> bool {
    if want.zero_sign_open {
        got == want.v
    } else {
        got.to_bits() == want.v.to_bits()
    }
}

/// Verdict first, then NaI's shape or the bracket bit for bit.
#[track_caller]
fn assert_bracket(door: &str, case: &str, got: Interval, want: Option<Want>) {
    let (lo_bits, hi_bits, dec) = got.repr_bits();
    let (lo, hi) = (f64::from_bits(lo_bits), f64::from_bits(hi_bits));
    assert_eq!(
        !got.is_certified(),
        want.is_none(),
        "{door}({case}): refusal verdict — door answered [{lo:e}, {hi:e}] dec {dec}"
    );
    match want {
        None => assert!(
            dec == 0 && lo.is_nan() && hi.is_nan(),
            "{door}({case}): a refusal is NaI — door answered [{lo:e}, {hi:e}] dec {dec}"
        ),
        Some(w) => {
            assert!(
                same_end(lo, w.lo) && same_end(hi, w.hi),
                "{door}({case}): endpoints — door [{lo:e}, {hi:e}], reference [{:e}, {:e}]",
                w.lo.v,
                w.hi.v
            );
            assert_eq!(
                dec,
                dec_code(w.dec),
                "{door}({case}): decoration — reference {:?}",
                w.dec
            );
        }
    }
}

/// Both verdicts reached, or the sweep says nothing about one of them.
fn assert_both_verdicts(door: &str, certified: usize, refused: usize) {
    assert!(
        certified > 0 && refused > 0,
        "{door}: the corpus reached {certified} certified and {refused} refused cases"
    );
}

#[test]
fn hull_is_its_reference_over_every_operand_pair() {
    let ops = corpus();
    let (mut certified, mut refused) = (0, 0);
    for a in &ops {
        for b in &ops {
            let want = hull_reference(a.back, b.back);
            if want.is_some() {
                certified += 1;
            } else {
                refused += 1;
            }
            assert_bracket(
                "hull",
                &format!("{a}, {b}"),
                Interval::hull(a.ring, b.ring),
                want,
            );
        }
    }
    assert_both_verdicts("hull", certified, refused);
}

#[test]
fn clamped_to_is_its_reference_over_every_operand_and_window() {
    let (mut certified, mut refused) = (0, 0);
    for x in corpus() {
        let bounds = probes(x.back);
        for &lo in &bounds {
            for &hi in &bounds {
                let want = clamp_reference(x.back, lo, hi);
                if want.is_some() {
                    certified += 1;
                } else {
                    refused += 1;
                }
                assert_bracket(
                    "clamped_to",
                    &format!("{x}, [{lo:e}, {hi:e}]"),
                    x.ring.clamped_to(lo, hi),
                    want,
                );
            }
        }
    }
    assert_both_verdicts("clamped_to", certified, refused);
}

// ------------------------------------------------------------ readings

/// `contains`: `p` is a real number lying in the bracket of an enclosure
/// that certifies.
fn contains_reference(x: DInterval, p: f64) -> bool {
    !refuses(x) && p.is_finite() && x.lo() <= p && p <= x.hi()
}

#[test]
fn contains_is_its_reference_over_every_operand_and_probe() {
    let (mut inside, mut outside) = (0, 0);
    for x in corpus() {
        let mut points = probes(x.back);
        points.extend(CORNERS.iter().chain(&EXTRAS));
        for p in points {
            let got = x.ring.contains(p);
            if refuses(x.back) {
                assert!(
                    !got,
                    "contains({x}, {p:e}): refusal verdict — a refused enclosure contains nothing"
                );
            }
            let want = contains_reference(x.back, p);
            assert_eq!(got, want, "contains({x}, {p:e}): membership");
            if want {
                inside += 1;
            } else {
                outside += 1;
            }
        }
    }
    assert_both_verdicts("contains", inside, outside);
}

/// `x` exactly, in units of the least subnormal `2^-1074`.
fn exact(x: f64) -> BigInt {
    let bits = x.to_bits();
    let field = (bits >> 52) & 0x7ff;
    let frac = bits & ((1 << 52) - 1);
    let units = if field == 0 {
        BigUint::from(frac)
    } else {
        BigUint::from(frac | (1 << 52)) << (field - 1)
    };
    let sign = if x.is_sign_negative() {
        Sign::Minus
    } else {
        Sign::Plus
    };
    BigInt::from_biguint(sign, units)
}

/// The low 64 bits of `x`, for a value its caller knows fits in them.
fn low_word(x: &BigUint) -> u64 {
    x.iter_u64_digits().next().unwrap_or(0)
}

/// The double nearest the exact value `d` (units of `2^-1074`), ties to
/// even, overflowing to infinity.
fn nearest(d: &BigInt) -> f64 {
    let mag = d.magnitude();
    let bits = mag.bits();
    // Below 2^53 units every value is a double, and its bit pattern is
    // the unit count itself (subnormal, or the least binade).
    let magnitude = if bits <= 53 {
        f64::from_bits(low_word(mag))
    } else {
        let mut shift = bits - 53;
        let mut q = low_word(&(mag >> shift));
        let rem = mag - (BigUint::from(q) << shift);
        let half = BigUint::from(1u8) << (shift - 1);
        if rem > half || (rem == half && q & 1 == 1) {
            q += 1;
        }
        if q == 1 << 53 {
            q >>= 1;
            shift += 1;
        }
        let field = shift + 1;
        if field >= 0x7ff {
            f64::INFINITY
        } else {
            f64::from_bits((field << 52) | (q - (1 << 52)))
        }
    };
    if d.sign() == Sign::Minus {
        -magnitude
    } else {
        magnitude
    }
}

/// `width`: NaN for a refusal, `+inf` for an unbounded side, `0` for a
/// point, and otherwise one step above the double nearest the exact
/// `hi − lo`.
fn width_reference(x: DInterval) -> f64 {
    if refuses(x) {
        return f64::NAN;
    }
    if x.lo() == f64::NEG_INFINITY || x.hi() == f64::INFINITY {
        return f64::INFINITY;
    }
    let d = exact(x.hi()) - exact(x.lo());
    if d.sign() == Sign::NoSign {
        return 0.0;
    }
    nearest(&d).next_up()
}

/// `mag`: NaN for a refusal, otherwise `max(|lo|, |hi|)`, exactly.
fn mag_reference(x: DInterval) -> f64 {
    if refuses(x) {
        return f64::NAN;
    }
    x.lo().abs().max(x.hi().abs())
}

#[track_caller]
fn assert_reading(door: &str, x: Operand, got: f64, want: f64) {
    assert_eq!(
        got.is_nan(),
        want.is_nan(),
        "{door}({x}): refusal verdict — door {got:e}, reference {want:e}"
    );
    if !want.is_nan() {
        assert_eq!(
            got.to_bits(),
            want.to_bits(),
            "{door}({x}): reading — door {got:e}, reference {want:e}"
        );
    }
}

#[test]
fn width_and_mag_are_their_references_over_every_operand() {
    let (mut certified, mut refused) = (0, 0);
    for x in corpus() {
        let w = width_reference(x.back);
        assert_reading("width", x, x.ring.width(), w);
        assert_reading("mag", x, x.ring.mag(), mag_reference(x.back));
        if w.is_nan() {
            refused += 1;
        } else {
            certified += 1;
        }
    }
    assert_both_verdicts("width and mag", certified, refused);
}
