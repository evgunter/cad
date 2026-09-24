//! **RING-2 review probes (R2 lane).** Three things the review had to
//! execute rather than read:
//!
//! 1. the crossing carries a refused scalar's real endpoints into
//!    certification arithmetic *as a refusal*, and every guard the spec keeps (`hull`,
//!    `clamped_to`, `contains`, `width`, `mag`, `powi(0)`) refuses it;
//! 2. the hazard the register names is real at the type: a refused
//!    quotient answers `true` to the comparison an unguarded consumer
//!    writes;
//! 3. the retired ring, ported here verbatim from the merge base, against
//!    the newtype op by op — how often the newtype is TIGHTER, how often
//!    it is LOOSER, and where. "Zero looser" is the PR's claim about
//!    the corpora; this row measures it about the arithmetic itself.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Bounds;
use geom_core::{CertifiedEnclosure, Interval, Real};
use test_utils::fuzz;

fn ri(lo: f64, hi: f64) -> Interval {
    Interval::from_bounds(lo, hi)
}

// ------------------------------------------------ 1. the crossing

#[test]
fn a_trv_scalar_with_real_endpoints_crosses_as_poison_with_its_endpoints() {
    // `sqrt([-1, 4])` clamps to `[0, 2]` at Trv: a sound bracket the
    // computation is not entitled to.
    let x = Interval::from_bounds(-1.0, 4.0).sqrt();
    assert!(x.certified_bracket().is_none(), "the fixture is a refusal");
    assert_eq!((x.lo(), x.hi()), (0.0, 2.0));
    let r = Interval::from_certified(x);
    assert!(!r.is_certified(), "the crossing must refuse: {r:?}");
    assert_eq!(
        (r.lo(), r.hi()),
        (0.0, 2.0),
        "and keep the endpoints: {r:?}"
    );
    assert!(r.certified_bracket().is_none());
    assert_eq!((r.lo(), r.hi()), (0.0, 2.0));

    // Every guard the spec keeps refuses it by name.
    let ok = ri(1.0, 2.0);
    assert!(!Interval::hull(r, ok).is_certified());
    assert!(!Interval::hull(ok, r).is_certified());
    assert!(!r.clamped_to(0.5, 1.5).is_certified());
    assert!(
        !r.clamped_to(-10.0, 10.0).is_certified(),
        "a window that changes nothing still refuses"
    );
    assert!(!r.contains(1.0));
    assert!(r.width().is_nan());
    assert!(r.mag().is_nan());
    assert!(!r.powi(0).is_certified(), "x^0 of a refusal is a refusal");
    assert!(!r.powi(2).is_certified() && !r.sqr().is_certified() && !(-r).is_certified());
    for y in [
        r + ok,
        ok + r,
        r - ok,
        ok - r,
        r * ok,
        ok * r,
        r / ok,
        ok / r,
    ] {
        assert!(!y.is_certified(), "{y:?}");
    }
    // A refused interval crossing into certification arithmetic keeps the
    // endpoints and the refusal.
    let rr = Interval::from_certified(r);
    assert!(
        !rr.is_certified() && (rr.lo(), rr.hi()) == (0.0, 2.0),
        "{rr:?}"
    );
    // The empty set and NaI cross as NaN-endpoint poison.
    let e =
        Interval::from_certified(Interval::from_bounds(1.0, 2.0) / Interval::from_bounds(0.0, 0.0));
    assert!(!e.is_certified() && e.lo().is_nan());
    // A certified scalar crosses clean, and its infinite side stays a bound.
    let c = Interval::from_certified(Interval::from_bounds(1.0, f64::INFINITY));
    assert!(c.is_certified() && c.hi().is_infinite(), "{c:?}");
    // A certified f64 at +inf is not a real: poison (the merge base's answer too).
    assert!(!Interval::from_certified(f64::INFINITY).is_certified());
    assert!(Interval::from_certified(2.5f64).is_certified());
}

// ------------------------------------------------ 2. the hazard

#[test]
fn a_refused_quotient_answers_true_to_the_unguarded_comparison() {
    let q = ri(-2.0, -1.0) / ri(0.0, 5e-324);
    assert!(!q.is_certified());
    // The consumer spelling the register lists at 20 sites: TRUE on a
    // value that refuses. This is what every `is_certified()`-first
    // rewrite exists for.
    assert!(q.hi() < 0.0, "{q:?}");
    assert!(q.hi().is_finite(), "{q:?}");
    // `f64::max(lo, 0.0)` no longer absorbs it either.
    let p = ri(1.0, 2.0) / ri(0.0, 5e-324);
    assert!(
        !p.is_certified() && p.lo().max(0.0) > 0.0 && p.lo().is_finite(),
        "{p:?}"
    );
    // A finite two-sided refusal: the midpoint hazard.
    let f = (ri(-2.0, -1.0) / ri(-1.0, 1.0)) * Interval::zero();
    assert!(
        !f.is_certified() && ((f.lo() + f.hi()) * 0.5).is_finite(),
        "{f:?}"
    );
}

// --------------------------------- 3. the retired ring, ported

/// The merge base's `Interval` arithmetic (`ed93c4cde`'s
/// `ring_interval.rs`), ported verbatim onto a bare pair so the
/// newtype can be measured against the arithmetic it replaced.
mod old {
    #[derive(Clone, Copy, Debug)]
    pub(super) struct Old {
        pub(super) lo: f64,
        pub(super) hi: f64,
    }
    fn down1(x: f64) -> f64 {
        x.next_down()
    }
    fn up1(x: f64) -> f64 {
        x.next_up()
    }
    impl Old {
        pub(super) fn poison() -> Self {
            Self {
                lo: f64::NAN,
                hi: f64::NAN,
            }
        }
        pub(super) fn is_poison(self) -> bool {
            self.lo.is_nan() || self.hi.is_nan()
        }
        #[allow(clippy::neg_cmp_op_on_partial_ord)]
        pub(super) fn from_bounds(lo: f64, hi: f64) -> Self {
            if !(lo <= hi) || lo == f64::INFINITY || hi == f64::NEG_INFINITY {
                return Self::poison();
            }
            Self { lo, hi }
        }
        fn is_exact_zero(self) -> bool {
            self.lo == 0.0 && self.hi == 0.0
        }
        pub(super) fn zero() -> Self {
            Self { lo: 0.0, hi: 0.0 }
        }
        pub(super) fn one() -> Self {
            Self { lo: 1.0, hi: 1.0 }
        }
        pub(super) fn sqr(self) -> Self {
            if self.is_poison() {
                return Self::poison();
            }
            if self.is_exact_zero() {
                return Self::zero();
            }
            if self.lo >= 0.0 {
                finish(down1(self.lo * self.lo).max(0.0), up1(self.hi * self.hi))
            } else if self.hi <= 0.0 {
                finish(down1(self.hi * self.hi).max(0.0), up1(self.lo * self.lo))
            } else {
                let (a, b) = (self.lo * self.lo, self.hi * self.hi);
                finish(0.0, up1(if a > b { a } else { b }))
            }
        }
        pub(super) fn powi(self, n: i32) -> Self {
            if self.is_poison() {
                return Self::poison();
            }
            if n == 0 {
                return Self::one();
            }
            let mut result = Self::one();
            let mut seeded = false;
            let mut acc = self;
            let mut e = n.unsigned_abs();
            while e > 0 {
                if e & 1 == 1 {
                    result = if seeded { result.mul(acc) } else { acc };
                    seeded = true;
                }
                e >>= 1;
                if e > 0 {
                    acc = acc.sqr();
                }
            }
            if n < 0 {
                Self::one().div(result)
            } else {
                result
            }
        }
        pub(super) fn neg(self) -> Self {
            if self.is_poison() {
                return Self::poison();
            }
            finish(-self.hi, -self.lo)
        }
        pub(super) fn add(self, rhs: Self) -> Self {
            if self.is_poison() || rhs.is_poison() {
                return Self::poison();
            }
            finish(down1(self.lo + rhs.lo), up1(self.hi + rhs.hi))
        }
        pub(super) fn sub(self, rhs: Self) -> Self {
            if self.is_poison() || rhs.is_poison() {
                return Self::poison();
            }
            finish(down1(self.lo - rhs.hi), up1(self.hi - rhs.lo))
        }
        pub(super) fn mul(self, rhs: Self) -> Self {
            if self.is_poison() || rhs.is_poison() {
                return Self::poison();
            }
            if self.is_exact_zero() || rhs.is_exact_zero() {
                return Self::zero();
            }
            let (lo, hi) = corner_min_max([
                self.lo * rhs.lo,
                self.lo * rhs.hi,
                self.hi * rhs.lo,
                self.hi * rhs.hi,
            ]);
            let (lo, hi) = sign_clamp(self, rhs, down1(lo), up1(hi));
            finish(lo, hi)
        }
        pub(super) fn div(self, rhs: Self) -> Self {
            if self.is_poison() || rhs.is_poison() {
                return Self::poison();
            }
            if !(rhs.lo > 0.0 || rhs.hi < 0.0) {
                return Self::poison();
            }
            if self.is_exact_zero() {
                return Self::zero();
            }
            let (lo, hi) = corner_min_max([
                self.lo / rhs.lo,
                self.lo / rhs.hi,
                self.hi / rhs.lo,
                self.hi / rhs.hi,
            ]);
            let (lo, hi) = sign_clamp(self, rhs, down1(lo), up1(hi));
            finish(lo, hi)
        }
    }
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn finish(lo: f64, hi: f64) -> Old {
        if !(lo <= hi) {
            return Old::poison();
        }
        Old { lo, hi }
    }
    fn corner_min_max(c: [f64; 4]) -> (f64, f64) {
        let (mut lo, mut hi) = (c[0], c[0]);
        for v in c {
            if v.is_nan() {
                return (f64::NAN, f64::NAN);
            }
            if v < lo {
                lo = v;
            }
            if v > hi {
                hi = v;
            }
        }
        (lo, hi)
    }
    fn sign_clamp(a: Old, b: Old, lo: f64, hi: f64) -> (f64, f64) {
        let (mut lo, mut hi) = (lo, hi);
        let nonneg = (a.lo >= 0.0 && b.lo >= 0.0) || (a.hi <= 0.0 && b.hi <= 0.0);
        let nonpos = (a.lo >= 0.0 && b.hi <= 0.0) || (a.hi <= 0.0 && b.lo >= 0.0);
        if nonneg && lo < 0.0 {
            lo = 0.0;
        }
        if nonpos && hi > 0.0 {
            hi = 0.0;
        }
        (lo, hi)
    }
}

use old::Old;

#[derive(Default, Debug)]
struct Count {
    identical: u64,
    tighter: u64,
    looser: u64,
    /// Neither contains the other.
    crossed: u64,
    /// One side poison and the other not.
    verdict_differs: u64,
    both_poison: u64,
    worst_looser: Option<(f64, f64, f64, f64, f64, f64)>,
}

fn classify(c: &mut Count, new: Interval, o: Old, a: (f64, f64), b: (f64, f64)) {
    match (!new.is_certified(), o.is_poison()) {
        (true, true) => c.both_poison += 1,
        (true, false) | (false, true) => c.verdict_differs += 1,
        (false, false) => {
            let (nl, nh, ol, oh) = (new.lo(), new.hi(), o.lo, o.hi);
            let sub = nl >= ol && nh <= oh;
            let sup = ol >= nl && oh <= nh;
            if sub && sup {
                c.identical += 1;
            } else if sub {
                c.tighter += 1;
            } else if sup {
                c.looser += 1;
                let excess = (ol - nl).max(nh - oh);
                if c.worst_looser.is_none_or(|w| excess > w.0) {
                    c.worst_looser = Some((excess, a.0, a.1, b.0, b.1, nl));
                }
            } else {
                c.crossed += 1;
            }
        }
    }
}

/// A finite value in a moderate exponent window, the regime the
/// certification code runs in.
fn moderate(rng: &mut fuzz::Rng) -> f64 {
    let m = rng.next_u64() & 0xf_ffff_ffff_ffff;
    let e = (1023 + (rng.next_u64() % 121) as i32 - 60) as u64;
    let s = rng.next_u64() & (1 << 63);
    f64::from_bits(s | (e << 52) | m)
}

/// The corners where an absolute pad and a sign clamp differ.
const EDGES: [f64; 12] = [
    -1.0,
    -f64::MIN_POSITIVE,
    -1e-160,
    -5e-324,
    -0.0,
    0.0,
    5e-324,
    1e-160,
    f64::MIN_POSITIVE,
    1.0,
    1e160,
    1e300,
];

fn draw(rng: &mut fuzz::Rng, edge: bool) -> (f64, f64) {
    let (a, b) = if edge {
        (EDGES[rng.below(EDGES.len())], EDGES[rng.below(EDGES.len())])
    } else {
        (moderate(rng), moderate(rng))
    };
    if a <= b { (a, b) } else { (b, a) }
}

/// The newtype against the retired ring on every shared op, over a
/// moderate corpus and an edge corpus, counted by direction.
#[test]
fn the_newtype_against_the_retired_ring_op_by_op() {
    let mut rng = fuzz::start("ring2-r2-probes");
    let ops = [
        "add", "sub", "mul", "div", "sqr", "neg", "powi3", "powi4", "powi-1", "powi-2",
    ];
    for edge in [false, true] {
        let mut counts: Vec<Count> = ops.iter().map(|_| Count::default()).collect();
        for _ in 0..fuzz::scaled(20_000) {
            let a = draw(&mut rng, edge);
            let b = draw(&mut rng, edge);
            let (na, nb) = (ri(a.0, a.1), ri(b.0, b.1));
            let (oa, ob) = (Old::from_bounds(a.0, a.1), Old::from_bounds(b.0, b.1));
            let pairs: [(Interval, Old); 10] = [
                (na + nb, oa.add(ob)),
                (na - nb, oa.sub(ob)),
                (na * nb, oa.mul(ob)),
                (na / nb, oa.div(ob)),
                (na.sqr(), oa.sqr()),
                (-na, oa.neg()),
                (na.powi(3), oa.powi(3)),
                (na.powi(4), oa.powi(4)),
                (na.powi(-1), oa.powi(-1)),
                (na.powi(-2), oa.powi(-2)),
            ];
            for (i, (n, o)) in pairs.into_iter().enumerate() {
                classify(&mut counts[i], n, o, a, b);
            }
        }
        let corpus = if edge {
            "edge corpus"
        } else {
            "moderate corpus"
        };
        for (op, c) in ops.iter().zip(&counts) {
            println!(
                "[{corpus}] {op:7} identical {:6} tighter {:6} LOOSER {:6} crossed {:6} \
                 verdict-differs {:6} both-poison {:6} worst-looser {:?}",
                c.identical,
                c.tighter,
                c.looser,
                c.crossed,
                c.verdict_differs,
                c.both_poison,
                c.worst_looser
            );
        }
        // The moderate corpus is where every corpus in the tree lives:
        // there the newtype is never looser and never crossed, and the
        // verdicts agree — the monotonicity argument, measured.
        if !edge {
            for (op, c) in ops.iter().zip(&counts) {
                assert_eq!((c.looser, c.crossed), (0, 0), "{op}: {c:?}");
            }
            for (op, c) in ops.iter().zip(&counts).take(6) {
                assert_eq!(c.verdict_differs, 0, "{op}: {c:?}");
            }
        }
    }
}

/// The one place the retired ring was TIGHTER than the backend, pinned
/// as a number: the sign clamp pulled an underflowed same-signed
/// product's lower bound back to `0`, and the backend pads it to
/// `-5e-324`. Absolute, one subnormal step, and only where a product
/// underflows.
#[test]
fn the_sign_clamps_one_subnormal_step_is_the_only_direction_the_newtype_gives_back() {
    let t = f64::MIN_POSITIVE;
    let n = ri(t, t) * ri(t, t);
    let o = Old::from_bounds(t, t).mul(Old::from_bounds(t, t));
    println!("newtype {n:?} retired {o:?}");
    assert_eq!(o.lo, 0.0, "the retired clamp answered exactly 0");
    assert!(n.lo() <= 0.0 && n.lo() >= -5e-324, "{n:?}");
    // …and the register's negative-powi class: the positive power pads
    // into zero, so the reciprocal refuses where the retired ring did not.
    let p = ri(1e-160, 1e-160).powi(-2);
    let q = Old::from_bounds(1e-160, 1e-160).powi(-2);
    println!("powi(-2) at 1e-160: newtype {p:?} retired {q:?}");

    // The four corners the differential pins the newtype's side of
    // (`interval_backend_differential::the_subnormal_and_overflow_corners_are_where_the_newtype_gives_width_back`),
    // measured HERE against the retired ring itself, which is the
    // only place in the tree that still has one.
    let sq = ri(t, t).sqr();
    let osq = Old::from_bounds(t, t).sqr();
    println!("sqr: newtype {sq:?} retired {osq:?}");
    assert_eq!((osq.lo, osq.hi), (0.0, 5e-324));
    assert_eq!(
        (sq.lo(), sq.hi()),
        (0.0, 1e-323),
        "one subnormal step wider at the top"
    );

    let p4 = ri(t, t).powi(4);
    let op4 = Old::from_bounds(t, t).powi(4);
    assert_eq!((op4.lo, op4.hi), (0.0, 5e-324));
    assert_eq!(
        (p4.lo(), p4.hi()),
        (0.0, 1e-323),
        "the same step, through powi(4)"
    );

    let recip = ri(t, 1e-160).powi(-1);
    let orecip = Old::from_bounds(t, 1e-160).powi(-1);
    println!("powi(-1) at [MIN_POSITIVE, 1e-160]: newtype {recip:?} retired {orecip:?}");
    assert_eq!(recip.lo(), orecip.lo, "the low end is shared");
    assert_eq!(
        (orecip.hi, recip.hi()),
        (4.494_232_837_155_791e307, 4.494_232_837_155_792e307),
        "one top-of-range ulp, the only non-subnormal direction the newtype gives back"
    );
}
