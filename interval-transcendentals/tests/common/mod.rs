//! Shared harness support: structured interval generators and the
//! containment/decoration/tightness checkers against the inari(+MPFR)
//! oracle.
//!
//! Split by feature: everything that mentions `inari` is behind
//! `oracle-inari`, so the oracle-free half (the generators) is
//! available to the tier of tests the kernel's CI runs without a C
//! toolchain. See the crate README's "Certification" section.
//!
//! The PRNG is **not** here. It used to be — a local SplitMix64 whose
//! every caller named a literal seed — and it is now `test_utils::fuzz`,
//! the same stream, dial and per-run seed the rest of the tree draws
//! from, which `review_fuzz_exact.rs` already used. What is left below is
//! the part that is actually specific to this crate: how to shape a
//! random interval.

#![allow(dead_code)] // shared by multiple integration-test binaries; each uses a subset
#[cfg(feature = "oracle-inari")]
use inari::DecInterval;
use interval_transcendentals::DInterval;
#[cfg(feature = "oracle-inari")]
use interval_transcendentals::Decoration;
use test_utils::fuzz;

/// Signed, log-uniform magnitude in [2^emin, 2^emax): stresses
/// subnormals through huge values evenly per binade.
///
/// A free function rather than a method because the RNG is now the
/// shared `fuzz::Rng`, and this shaping is this crate's business, not
/// the harness's.
pub fn log_mag(rng: &mut fuzz::Rng, emin: i32, emax: i32) -> f64 {
    let e = rng.range(f64::from(emin), f64::from(emax));
    let m = 1.0 + rng.unit();
    let s = if rng.next_u64() & 1 == 0 { 1.0 } else { -1.0 };
    s * m * libm::exp2(e)
}

/// One structured random interval: mixes points, ulp-tight, moderate,
/// wide, extremum-straddling (near k·π/2), zero-touching, and
/// signed-zero cases, over the magnitude window [2^emin, 2^emax).
pub fn gen_interval(rng: &mut fuzz::Rng, emin: i32, emax: i32) -> DInterval {
    let a = log_mag(rng, emin, emax);
    match rng.next_u64() % 8 {
        0 => DInterval::point(a),
        1 => DInterval::from_bounds(a, a + a.abs() * 1e-15 + f64::MIN_POSITIVE),
        2 => DInterval::from_bounds(a.min(0.0), a.max(0.0)), // touches 0
        3 => {
            // Straddle a trig-critical point k·π/2, tight jitter.
            let k = (rng.next_u64() % 64) as f64 - 32.0;
            let c = k * core::f64::consts::FRAC_PI_2;
            let w = log_mag(rng, -40, 2).abs();
            DInterval::from_bounds(c - w, c + w)
        }
        4 => {
            let w = log_mag(rng, emin, emax).abs();
            DInterval::from_bounds(a, a + w)
        }
        5 => DInterval::from_bounds(-0.0, a.abs()),
        6 => DInterval::from_bounds(-a.abs(), 0.0),
        _ => {
            let b = log_mag(rng, emin, emax);
            DInterval::from_bounds(a.min(b), a.max(b))
        }
    }
}

/// The same interval as an inari `DecInterval` (constructor decorations
/// agree: Com bounded, Dac unbounded).
#[cfg(feature = "oracle-inari")]
pub fn to_inari(x: &DInterval) -> DecInterval {
    DecInterval::try_from((x.lo(), x.hi())).expect("generator produced valid bounds")
}

#[cfg(feature = "oracle-inari")]
pub fn dec_of(d: inari::Decoration) -> Decoration {
    match d {
        inari::Decoration::Ill => Decoration::Ill,
        inari::Decoration::Trv => Decoration::Trv,
        inari::Decoration::Def => Decoration::Def,
        inari::Decoration::Dac => Decoration::Dac,
        inari::Decoration::Com => Decoration::Com,
    }
}

/// Tightness accumulator: ratio of my width to oracle width where both
/// are finite, nonempty, and the oracle width is positive.
#[cfg(feature = "oracle-inari")]
#[derive(Default)]
pub struct Tightness {
    pub ratios: Vec<f64>,
    pub mine_wider_cases: u64,
    pub total: u64,
}

#[cfg(feature = "oracle-inari")]
impl Tightness {
    pub fn record(&mut self, mine: &DInterval, oracle: &DecInterval) {
        self.total += 1;
        let (Some(iv), false) = (oracle.interval(), mine.is_empty()) else {
            return;
        };
        let (ow, mw) = (iv.wid(), mine.hi() - mine.lo());
        if ow.is_finite() && mw.is_finite() && ow > 0.0 {
            let r = mw / ow;
            self.ratios.push(r);
            if r > 1.0 {
                self.mine_wider_cases += 1;
            }
        }
    }

    /// Print the summary line `label: n mean p50 p99 max wider%`, and —
    /// where the caller supplies one — ASSERT a ceiling on the worst
    /// width ratio seen.
    ///
    /// The ceiling is the upper counterpart to `assert_contains`, in the
    /// tier that has an oracle. `assert_contains` alone gets EASIER as
    /// the enclosure degrades, so without this the printed distribution
    /// was the only record of tightness and nothing computed with it.
    /// `pad_contract.rs` bounds each PAD from above in the cheap tier;
    /// this bounds the whole enclosure against a correctly-rounded
    /// reference, which is what catches a widening that does not come
    /// from a pad — an extremum-capture rule that fires too often, a
    /// range clip dropped, a corner evaluation gone wide.
    ///
    /// **The ceiling is DERIVED and then confirmed, not fitted.** Our
    /// width exceeds a correctly-rounded one by at most `2·pad` steps,
    /// and the oracle's own width is at least one step wherever it is
    /// inexact, so the worst ratio is structurally about `2·pad + 1` —
    /// 3 for the 1-step ops, 9 for the 4-step ones. The ceilings the
    /// callers pass sit far above that (see `certify.rs`), and the
    /// measured maxima on this branch, at efforts 1 and 2 over four
    /// varying seeds, were 3 and 8–10 respectively.
    ///
    /// **`None` means the caller has a documented reason no ceiling
    /// applies**, not that it forgot: `powi`'s pad count is a function
    /// of its exponent rather than a constant, and the huge-magnitude
    /// window is where the crate's localization is DOCUMENTED to degrade
    /// to `[-1, 1]` (semantics-diffs D3), with observed ratios past
    /// 2^70. Bounding either would be a guard that fires on a sound
    /// enclosure.
    pub fn report(&mut self, label: &str, max_ratio: Option<f64>) {
        self.ratios.sort_unstable_by(f64::total_cmp);
        let n = self.ratios.len();
        if n == 0 {
            println!(
                "TIGHTNESS {label}: no finite-ratio samples of {} total",
                self.total
            );
            return;
        }
        let worst = self.ratios[n - 1];
        if let Some(cap) = max_ratio {
            assert!(
                worst <= cap,
                "TIGHTNESS CEILING EXCEEDED for {label}: worst width ratio \
                 {worst} vs oracle exceeds {cap} over n={n} samples. The \
                 enclosure got wider; find out why before touching this \
                 number, and if the new width is right, re-derive the \
                 ceiling rather than restoring the old one."
            );
        }
        let mean: f64 = self.ratios.iter().sum::<f64>() / n as f64;
        let p = |q: f64| self.ratios[((n - 1) as f64 * q) as usize];
        println!(
            "TIGHTNESS {label}: n={n} mean={mean:.6} p50={:.6} p99={:.6} max={:.6} wider={:.3}%",
            p(0.5),
            p(0.99),
            worst,
            100.0 * self.mine_wider_cases as f64 / self.total as f64
        );
    }
}

/// The containment core: the oracle's enclosure (which contains truth)
/// must be a subset of ours; empty/NaI classifications must agree.
/// Decoration soundness: ours must not exceed the oracle's, EXCEPT in
/// enumerated classes where the oracle is known-conservative and our
/// stronger decoration is proven correct (docs/semantics-diffs.md);
/// callers pass `dec_exception` for those.
#[cfg(feature = "oracle-inari")]
pub fn assert_contains(ctx: &str, mine: &DInterval, oracle: &DecInterval, dec_exception: bool) {
    if oracle.is_nai() {
        assert!(mine.is_nai(), "{ctx}: oracle NaI, mine {mine:?}");
        return;
    }
    let iv = oracle.interval().expect("non-NaI oracle has an interval");
    if iv.is_empty() {
        assert!(mine.is_empty(), "{ctx}: oracle empty, mine {mine:?}");
        return;
    }
    assert!(
        !mine.is_empty() && !mine.is_nai(),
        "{ctx}: oracle {oracle:?}, mine {mine:?}"
    );
    assert!(
        mine.lo() <= iv.inf() && iv.sup() <= mine.hi(),
        "{ctx}: CONTAINMENT VIOLATION oracle=[{:e},{:e}] mine=[{:e},{:e}]",
        iv.inf(),
        iv.sup(),
        mine.lo(),
        mine.hi()
    );
    if !dec_exception {
        assert!(
            mine.decoration() <= dec_of(oracle.decoration()),
            "{ctx}: decoration {:?} exceeds oracle {:?}",
            mine.decoration(),
            oracle.decoration()
        );
    }
}
