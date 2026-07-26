//! Shared harness support: seed-pinned PRNG, structured interval
//! generators, and the containment/decoration/tightness checkers against
//! the inari(+MPFR) oracle.

#![allow(dead_code)] // shared by multiple integration-test binaries; each uses a subset
use inari::DecInterval;
use interval_transcendentals::{DInterval, Decoration};

/// SplitMix64: tiny, seed-pinned, dependency-free. Every test names its
/// own constant seed; runs are bit-reproducible.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }

    /// Uniform in [lo, hi).
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.unit() * (hi - lo)
    }

    /// Signed, log-uniform magnitude in [2^emin, 2^emax): stresses
    /// subnormals through huge values evenly per binade.
    pub fn log_mag(&mut self, emin: i32, emax: i32) -> f64 {
        let e = self.range(f64::from(emin), f64::from(emax));
        let m = 1.0 + self.unit();
        let s = if self.next_u64() & 1 == 0 { 1.0 } else { -1.0 };
        s * m * libm::exp2(e)
    }
}

/// One structured random interval: mixes points, ulp-tight, moderate,
/// wide, extremum-straddling (near k·π/2), zero-touching, and
/// signed-zero cases, over the magnitude window [2^emin, 2^emax).
pub fn gen_interval(rng: &mut Rng, emin: i32, emax: i32) -> DInterval {
    let a = rng.log_mag(emin, emax);
    match rng.next_u64() % 8 {
        0 => DInterval::point(a),
        1 => DInterval::from_bounds(a, a + a.abs() * 1e-15 + f64::MIN_POSITIVE),
        2 => DInterval::from_bounds(a.min(0.0), a.max(0.0)), // touches 0
        3 => {
            // Straddle a trig-critical point k·π/2, tight jitter.
            let k = (rng.next_u64() % 64) as f64 - 32.0;
            let c = k * core::f64::consts::FRAC_PI_2;
            let w = rng.log_mag(-40, 2).abs();
            DInterval::from_bounds(c - w, c + w)
        }
        4 => {
            let w = rng.log_mag(emin, emax).abs();
            DInterval::from_bounds(a, a + w)
        }
        5 => DInterval::from_bounds(-0.0, a.abs()),
        6 => DInterval::from_bounds(-a.abs(), 0.0),
        _ => {
            let b = rng.log_mag(emin, emax);
            DInterval::from_bounds(a.min(b), a.max(b))
        }
    }
}

/// The same interval as an inari `DecInterval` (constructor decorations
/// agree: Com bounded, Dac unbounded).
pub fn to_inari(x: &DInterval) -> DecInterval {
    DecInterval::try_from((x.lo(), x.hi())).expect("generator produced valid bounds")
}

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
#[derive(Default)]
pub struct Tightness {
    pub ratios: Vec<f64>,
    pub mine_wider_cases: u64,
    pub total: u64,
}

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

    /// Print summary line: `label: n mean p50 p99 max wider%`.
    pub fn report(&mut self, label: &str) {
        self.ratios.sort_unstable_by(f64::total_cmp);
        let n = self.ratios.len();
        if n == 0 {
            println!(
                "TIGHTNESS {label}: no finite-ratio samples of {} total",
                self.total
            );
            return;
        }
        let mean: f64 = self.ratios.iter().sum::<f64>() / n as f64;
        let p = |q: f64| self.ratios[((n - 1) as f64 * q) as usize];
        println!(
            "TIGHTNESS {label}: n={n} mean={mean:.6} p50={:.6} p99={:.6} max={:.6} wider={:.3}%",
            p(0.5),
            p(0.99),
            self.ratios[n - 1],
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
pub fn assert_contains(ctx: &str, mine: &DInterval, oracle: &DecInterval, dec_exception: bool) {
    if oracle.is_nai() {
        assert!(mine.is_nai(), "{ctx}: oracle NaI, mine {mine:?}");
        return;
    }
    let iv = oracle.interval().expect("non-NaI oracle has an interval");
    if iv.is_empty() {
        assert!(
            mine.is_empty() || !mine.is_nai(),
            "{ctx}: oracle empty, mine NaI"
        );
        assert!(
            mine.is_empty(),
            "{ctx}: oracle empty, mine nonempty {mine:?}"
        );
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
