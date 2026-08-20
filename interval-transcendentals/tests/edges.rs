//! Edge sweeps: poison propagation, signed zeros, extremum capture,
//! domain clamps, huge arguments, and the specific traps found during
//! design (atan2 −0.0 branch cut; powi straddle-square tightness).
//!
//! This file runs in BOTH certification tiers. Its primary assertions are
//! about our own values and need no oracle, so the kernel's CI can run it
//! without a C toolchain; the inari cross-checks interleaved with them are
//! behind `oracle-inari` (README "Certification"). Nothing was deleted to
//! achieve that — the oracle rows are simply conditional.

mod common;

#[cfg(feature = "oracle-inari")]
use common::{assert_contains, dec_of, to_inari};
use interval_transcendentals::{DInterval, Decoration};

#[test]
fn poison_propagation() {
    let nai = DInterval::point(f64::NAN);
    assert!(nai.is_nai(), "NaN constructs NaI");
    assert!(
        DInterval::point(f64::INFINITY).is_nai(),
        "inf constructs NaI"
    );
    assert!(DInterval::from_bounds(3.0, 2.0).is_nai(), "inverted bounds");
    let x = DInterval::from_bounds(1.0, 2.0);
    for r in [
        nai.sin(),
        nai.atan2(x),
        x.atan2(nai),
        (nai + x).sqrt(),
        nai.powi(0),
    ] {
        assert!(r.is_nai(), "NaI must propagate, got {r:?}");
    }
    let empty = DInterval::empty();
    for r in [
        empty.sin(),
        empty.tan(),
        empty.sqrt(),
        empty.powi(0),
        empty.atan2(x),
    ] {
        assert!(r.is_empty(), "empty must propagate, got {r:?}");
    }
}

#[test]
fn sin_extremum_capture() {
    // Interval containing π/2 must reach exactly 1.0 (and no further).
    let x = DInterval::from_bounds(1.5, 1.6);
    let s = x.sin();
    assert_eq!(s.hi(), 1.0, "sup must be the exact extremum, got {s:?}");
    // One-sided by design: this row is about the CAPTURE. The other
    // side of the same endpoint — that the pad is no wider than derived
    // — is `pad_contract.rs`, on inputs chosen so no capture fires.
    assert!(s.lo() <= libm::sin(1.5) && s.decoration() == Decoration::Com);
    // Same through the oracle lens.
    #[cfg(feature = "oracle-inari")]
    assert_contains("sin ext", &s, &to_inari(&x).sin(), false);
    // Width ≥ 2π: both extrema.
    let w = DInterval::from_bounds(100.0, 107.0).sin();
    assert_eq!((w.lo(), w.hi()), (-1.0, 1.0));
}

#[test]
fn tan_pole_refusal() {
    let pole = DInterval::from_bounds(1.5, 1.6); // contains π/2
    let t = pole.tan();
    assert_eq!(t.decoration(), Decoration::Trv, "pole ⇒ Trv, got {t:?}");
    assert!(t.lo() == f64::NEG_INFINITY && t.hi() == f64::INFINITY);
    // Pole-free branch stays Com and encloses the endpoint values.
    // "Tight-ish" is not asserted here and cannot be: the bound on how
    // far out those endpoints may sit is `pad_contract.rs`'s.
    let ok = DInterval::from_bounds(-0.5, 0.5).tan();
    assert_eq!(ok.decoration(), Decoration::Com);
    assert!(ok.lo() <= libm::tan(-0.5) && ok.hi() >= libm::tan(0.5));
}

#[test]
fn huge_argument_honesty() {
    // |x| ~ 1e300: sin/cos widen to [-1,1] (sound), tan refuses (Trv).
    let x = DInterval::from_bounds(1e300, 1e300);
    let (s, c) = x.sin_cos();
    assert_eq!((s.lo(), s.hi(), c.lo(), c.hi()), (-1.0, 1.0, -1.0, 1.0));
    assert_eq!(
        s.decoration(),
        Decoration::Com,
        "sin defined everywhere: no false poison"
    );
    let t = x.tan();
    assert_eq!(t.decoration(), Decoration::Trv);
    // Moderate arguments still localize: sin near 1e6 is thin.
    let m = DInterval::from_bounds(1e6, 1e6 + 1e-3);
    let sm = m.sin();
    assert!(
        sm.hi() - sm.lo() < 2e-3,
        "moderate args must stay tight, got {sm:?}"
    );
    #[cfg(feature = "oracle-inari")]
    assert_contains("sin 1e6", &sm, &to_inari(&m).sin(), false);
}

#[test]
fn domain_clamps_poison_not_decide() {
    // Partial miss: clamp + Trv (M0 ruling: clamps never decide).
    let part = DInterval::from_bounds(-1.0, 4.0).sqrt();
    assert_eq!(part.decoration(), Decoration::Trv);
    assert_eq!(part.lo(), 0.0);
    assert!(part.hi() >= 2.0);
    // Full miss: empty.
    assert!(DInterval::from_bounds(-4.0, -1.0).sqrt().is_empty());
    let a = DInterval::from_bounds(0.5, 2.0).asin();
    assert_eq!(a.decoration(), Decoration::Trv);
    assert!(a.hi() >= core::f64::consts::FRAC_PI_2);
    assert!(DInterval::from_bounds(1.5, 2.0).acos().is_empty());
    // In-domain: no poison.
    assert_eq!(
        DInterval::from_bounds(-0.5, 0.5).asin().decoration(),
        Decoration::Com
    );
}

#[test]
fn atan2_branch_cut_and_signed_zero() {
    // The unsoundness trap: y = [-0.0, 3] is the real set [0, 3]; against
    // x < 0 the value π (at y = 0) MUST be inside.
    let y = DInterval::from_bounds(-0.0, 3.0);
    let x = DInterval::from_bounds(-2.0, -1.0);
    let r = y.atan2(x);
    assert!(
        r.contains(core::f64::consts::PI),
        "π must be contained, got {r:?}"
    );
    assert!(r.decoration() >= Decoration::Dac, "continuous on this box");
    // Cut crossed: full hull, Def.
    let yc = DInterval::from_bounds(-1.0, 1.0);
    let rc = yc.atan2(x);
    assert!(rc.contains(core::f64::consts::PI) && rc.contains(-core::f64::consts::PI));
    assert_eq!(rc.decoration(), Decoration::Def);
    // Origin in box: Trv. Degenerate origin: empty.
    let z = DInterval::from_bounds(-0.0, 0.0);
    assert_eq!(yc.atan2(z).decoration(), Decoration::Trv);
    assert!(z.atan2(z).is_empty());
    // Oracle agreement on all four.
    #[cfg(feature = "oracle-inari")]
    for (yy, xx) in [(y, x), (yc, x), (yc, z), (z, z)] {
        let mine = yy.atan2(xx);
        let oracle = to_inari(&yy).atan2(to_inari(&xx));
        let exception = xx.hi() < 0.0 && yy.lo() == 0.0;
        assert_contains("atan2 edge", &mine, &oracle, exception);
    }
}

#[test]
fn powi_straddle_square_is_tight_at_zero() {
    // The tight-square contract: even powers of straddling
    // intervals have lower bound EXACTLY 0.0 — sqrt downstream stays Com.
    let x = DInterval::from_bounds(-3.0, 2.0);
    let sq = x.powi(2);
    assert_eq!(sq.lo(), 0.0, "no spurious negative lo: {sq:?}");
    assert_eq!(sq.decoration(), Decoration::Com);
    assert_eq!(
        sq.sqrt().decoration(),
        Decoration::Com,
        "no poison laundered in"
    );
    assert!(sq.hi() >= 9.0 && sq.hi() <= 9.0_f64.next_up().next_up());
    // n = 0 propagates poison but is [1,1] on clean input.
    let one = x.powi(0);
    assert_eq!((one.lo(), one.hi()), (1.0, 1.0));
    // Negative exponent over zero-straddle: unbounded + Trv via division.
    let inv = x.powi(-1);
    assert_eq!(inv.decoration(), Decoration::Trv);
    // Exact squares stay exact (sqrt witness path).
    let four = DInterval::from_bounds(4.0, 4.0).sqrt();
    assert_eq!((four.lo(), four.hi()), (2.0, 2.0));
}

#[test]
fn subnormal_and_zero_endpoints() {
    let tiny = DInterval::from_bounds(0.0, f64::MIN_POSITIVE);
    // Oracle-free: subnormal endpoints must survive as enclosures at all,
    // with sqrt's clamp-free `Com` and sin's identity-at-zero bracket.
    assert!(tiny.sin().contains(0.0) && tiny.sqrt().contains(0.0));
    assert_eq!(tiny.sqrt().decoration(), Decoration::Com);
    assert!(tiny.atan().contains(0.0));
    #[cfg(feature = "oracle-inari")]
    for (mine, oracle) in [
        (tiny.sin(), to_inari(&tiny).sin()),
        (tiny.sqrt(), to_inari(&tiny).sqrt()),
        (tiny.atan(), to_inari(&tiny).atan()),
    ] {
        assert_contains("subnormal", &mine, &oracle, false);
        assert_eq!(mine.decoration(), dec_of(oracle.decoration()));
    }
    let nz = DInterval::from_bounds(-0.0, -0.0);
    let s = nz.sin();
    assert!(s.contains(0.0) && s.decoration() == Decoration::Com);
}

#[test]
fn floor_d8_divergence_is_real_and_bounded() {
    // D8 (semantics-diffs.md): constant-on-box floor with integer left
    // endpoint — we say Com (restriction is constant), inari says Dac
    // (ambient discontinuity at the endpoint). Pin BOTH sides so the
    // allowlist in certify_exact_ops can never silently widen.
    let s = DInterval::from_bounds(3.0, 3.0);
    assert_eq!(s.floor().decoration(), Decoration::Com);
    // Our side of the non-D8 shapes, oracle-free.
    let c = DInterval::from_bounds(2.25, 2.75);
    assert_eq!(c.floor().decoration(), Decoration::Com);
    let j = DInterval::from_bounds(2.25, 3.25);
    assert_eq!(j.floor().decoration(), Decoration::Def);
    // The oracle half: D8 is a real divergence and the other two agree.
    #[cfg(feature = "oracle-inari")]
    {
        assert_eq!(
            to_inari(&s).floor().decoration(),
            inari::Decoration::Dac,
            "inari changed its floor decoration policy; revisit D8"
        );
        assert_eq!(
            dec_of(to_inari(&c).floor().decoration()),
            c.floor().decoration()
        );
        assert_eq!(
            dec_of(to_inari(&j).floor().decoration()),
            j.floor().decoration()
        );
    }
}
