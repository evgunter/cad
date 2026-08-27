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

/// Signed distance in representable steps from `from` to `to`, counted
/// on the ladder `f64::next_up`/`next_down` walk — the ladder
/// `round.rs`'s `step_up`/`step_down` climb, so the number this returns
/// is the number of pad steps taken. `steps(x, x.next_up()) == 1` for
/// every finite `x`, and the two zeros are ONE rung, because that ladder
/// steps from `+0.0` straight to `-MIN_SUBNORMAL`.
///
/// One home, shared by `pad_contract.rs` (which pins it) and by
/// [`Tightness`] below. Unlike the derived pad constant, which
/// `pad_contract.rs` copies ON PURPOSE, this is a metric and not a
/// claim: sharing it cannot disarm anything.
pub fn steps(from: f64, to: f64) -> i128 {
    fn key(x: f64) -> i128 {
        assert!(x.is_finite(), "step distance is only defined on finites");
        let m = i128::from(x.abs().to_bits());
        if x < 0.0 { -m } else { m }
    }
    key(to) - key(from)
}

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
            //
            // The minimum jitter here is load-bearing for the oracle
            // tier: at 2^-40 (9.1e-13) it swallows, by 25.6x, the
            // <= 3.6e-14 band in which `tan`'s grid test refuses beside
            // a pole over this case's |k| <= 32. So these draws make the
            // ORACLE unbounded too and land in a bucket that carries no
            // assert. Narrowing it — or adding a
            // near-pole-but-pole-free class beside it — puts sound,
            // unbounded results under `Tightness`' zero-tolerance
            // assert on `mine_unbounded_oracle_bounded`. See that
            // field's docs before changing this exponent.
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

/// What a [`Tightness`] run is allowed to look like. Passing one turns
/// the accumulator from a printer into a guard; passing `None` to
/// [`Tightness::report`] leaves it a printer, and the caller owes a
/// reason at the call site.
#[cfg(feature = "oracle-inari")]
pub struct Ceiling {
    /// Upper bound on the worst width ratio vs the correctly-rounded
    /// oracle, over the samples where a ratio is meaningful.
    pub max_ratio: f64,
    /// Anti-vacuity: the least fraction of `total` that must yield a
    /// comparable ratio. Without this the ceiling is defeated by the
    /// degradation it exists to catch — an operation that regressed to
    /// `entire()` on every draw contributes no ratios at all, and a
    /// max-over-nothing passes. The new fuzz lanes in this crate carry
    /// floors of exactly this shape; so does this.
    ///
    /// Its reach, stated rather than assumed: a change to `src/` can
    /// only push cases into the `empty` bucket (which `assert_contains`
    /// already catches, on taxonomy agreement) or into
    /// `mine_unbounded_oracle_bounded` (which has its own assert
    /// immediately below). What this floor uniquely covers is the
    /// HARNESS — a generator that stopped producing comparable cases, or
    /// a `record` that started dropping them. Demonstrated: skewing
    /// `gen_interval` to 90% unbounded draws reds `add` at n=2 989 and
    /// `tan` at n=24 486.
    pub min_ratio_fraction: f64,
    /// Upper bound, in representable steps, on OUR width for the class
    /// the ratio cannot score: the oracle proved the value exact
    /// (`wid() == 0`) and we padded anyway. The ratio there is infinite
    /// and SOUND — an exactly-representable result still gets its
    /// outward pad — so the honest bound is absolute, not relative, and
    /// it is the one class where a width blow-up would otherwise be
    /// invisible to both instruments.
    pub max_steps_when_oracle_exact: i128,
}

/// Tightness accumulator. Every drawn case lands in exactly one bucket,
/// and the buckets that carry no ratio are COUNTED rather than dropped:
/// the shape this guard exists to catch shows up as an empty sample set,
/// so a silent drop is the defect one level up.
#[cfg(feature = "oracle-inari")]
#[derive(Default)]
pub struct Tightness {
    pub ratios: Vec<f64>,
    pub mine_wider_cases: u64,
    pub total: u64,
    /// Either side empty: no widths to compare.
    pub empty_cases: u64,
    /// The oracle's own enclosure is unbounded, so a ratio of two
    /// infinities says nothing about us.
    pub oracle_unbounded: u64,
    /// **Ours is unbounded while the oracle's is bounded.** The loudest
    /// degradation available, and a ratio cannot express it.
    ///
    /// **What keeps this at zero is NOT the window bounds** — that is
    /// the ratio ceiling's constraint, recorded on `certify.rs`'s
    /// `WINDOWS`, and pointing a reader there for this assert would send
    /// them to the wrong place. Two SOUND classes land in this bucket,
    /// and both sit deep inside the ceiling-carrying windows:
    ///
    /// 1. **`tan`'s honest pole refusal.** The conservative grid test
    ///    refuses for 2–5 ulps either side of the f64 image of every
    ///    pole `(k + 1/2)·π`, at every `k` from ±1 to ±651 — i.e.
    ///    `|x| ≈ 1.57 … 1022`, twenty-four binades BELOW the `2^32`
    ///    false-capture onset. `point(next_up(FRAC_PI_2)).tan()` returns
    ///    `[-inf, inf]` with `Trv` where inari returns a bounded
    ///    `≈ 6.2e15`. Sound, and unbounded.
    /// 2. **The overflow pad.** `[MAX, MAX] + [-1, -1]` pads up to
    ///    `+inf` where inari stays bounded; `certify_arith` and
    ///    `certify_atan2` sweep all four windows into one ceilinged
    ///    accumulator, so there is no exempt window to catch it.
    ///
    /// Neither is reachable from the shipped generator (0 hits in ~10x
    /// CI's effort-8 volume), and for class 1 that is a margin rather
    /// than luck: [`gen_interval`]'s pole-straddling case 3 has a
    /// minimum jitter of `2^-40 ≈ 9.1e-13`, and the refusal band is at
    /// most `3.6e-14` wide (5 representable steps, at `33·π/2`, the
    /// widest over that case's `|k| <= 32` — measured by walking outward
    /// from each pole until `tan` stops refusing). The jitter therefore
    /// swallows the band by **25.6x** and the draw lands in
    /// `oracle_unbounded` instead. **What protects this assert is the
    /// measure of that few-ulp band, and nothing about `emax`.**
    ///
    /// So: **a generator that added a deliberate near-pole-but-pole-free
    /// case class — an obvious thing to want in a trig harness — would
    /// red this assert on a sound enclosure.** That is the change to
    /// watch for, and the fix then is to route those draws to an
    /// accumulator with no ceiling, not to loosen this.
    pub mine_unbounded_oracle_bounded: u64,
    /// The oracle proved the value exact (`wid() == 0`); scored in
    /// representable steps instead of as a ratio.
    pub oracle_exact: u64,
    pub worst_steps_when_oracle_exact: i128,
}

#[cfg(feature = "oracle-inari")]
impl Tightness {
    pub fn record(&mut self, mine: &DInterval, oracle: &DecInterval) {
        self.total += 1;
        let (Some(iv), false) = (oracle.interval(), mine.is_empty()) else {
            self.empty_cases += 1;
            return;
        };
        if iv.is_empty() {
            self.empty_cases += 1;
            return;
        }
        let (ow, mw) = (iv.wid(), mine.hi() - mine.lo());
        if !ow.is_finite() {
            self.oracle_unbounded += 1;
            return;
        }
        if !mw.is_finite() {
            self.mine_unbounded_oracle_bounded += 1;
            return;
        }
        if ow == 0.0 {
            self.oracle_exact += 1;
            self.worst_steps_when_oracle_exact = self
                .worst_steps_when_oracle_exact
                .max(steps(mine.lo(), mine.hi()));
            return;
        }
        let r = mw / ow;
        self.ratios.push(r);
        if r > 1.0 {
            self.mine_wider_cases += 1;
        }
    }

    /// Print the distribution and the bucket census, then — where the
    /// caller supplies a [`Ceiling`] — assert it.
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
    /// **The line prints before the assert fires**, so a red run hands
    /// the reader the distribution the number was derived from instead
    /// of telling them to go and find it.
    ///
    /// **Where the ratio bound comes from, corrected.** Our width
    /// exceeds a correctly-rounded one by at most `2·pad` outward
    /// STEPS — but the ratio is on WIDTHS, and a step that crosses a
    /// binade boundary is worth twice the oracle's ulp (the factor
    /// `docs/derivations.md` §1 Lemma P2 carries and P3's step-counting
    /// route drops). So the structural worst is `≈ 4·pad + 1`: **17**
    /// for the 4-step ops and **5** for the 1-step ops, not 9 and 3.
    /// Measured maxima across seeds and efforts are 3 for the 1-step ops
    /// and 8–12 for the 4-step ones — inside 17, and the earlier
    /// derivation of 9 was the thing that was wrong, not the
    /// measurement. **Do not tighten the ceilings toward 9 or 3**; those
    /// numbers were never the bound.
    ///
    /// **The ceiling's exemption depends on a window boundary, not on
    /// the `4·10^15` figure the crate's prose quotes.** Ratios blow past
    /// any ceiling only through FALSE EXTREMUM CAPTURE, whose onset is
    /// `|x| ≈ 2^32` — partial degradation, seven binades before the
    /// total degradation at `2^52` that the prose describes. It is safe
    /// here only because `certify.rs`'s exempt window starts at `2^30`.
    /// **Widening a ceiling-carrying window's `emax` past 30 would
    /// produce a red on a sound enclosure**; that is the constraint, and
    /// it lives with the windows too.
    pub fn report(&mut self, label: &str, ceiling: Option<Ceiling>) {
        self.ratios.sort_unstable_by(f64::total_cmp);
        let n = self.ratios.len();
        let (mean, p50, p99, worst) = if n == 0 {
            (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
        } else {
            let p = |q: f64| self.ratios[((n - 1) as f64 * q) as usize];
            (
                self.ratios.iter().sum::<f64>() / n as f64,
                p(0.5),
                p(0.99),
                self.ratios[n - 1],
            )
        };
        println!(
            "TIGHTNESS {label}: n={n}/{} mean={mean:.6} p50={p50:.6} p99={p99:.6} \
             max={worst:.6} wider={:.3}% | empty={} oracle_unbounded={} \
             MINE_UNBOUNDED={} oracle_exact={} (worst {} steps)",
            self.total,
            100.0 * self.mine_wider_cases as f64 / self.total as f64,
            self.empty_cases,
            self.oracle_unbounded,
            self.mine_unbounded_oracle_bounded,
            self.oracle_exact,
            self.worst_steps_when_oracle_exact,
        );
        let Some(c) = ceiling else {
            return;
        };
        assert_eq!(
            self.mine_unbounded_oracle_bounded, 0,
            "{label}: {} draws returned an UNBOUNDED enclosure where the oracle's \
             was bounded. That is the widest a result can get, and a ratio cannot \
             say so — which is why it is counted. If this fired because the \
             GENERATOR changed, read the two sound classes documented on this \
             field first: a near-pole-but-pole-free trig draw, or an \
             overflow-padded endpoint, is sound here and belongs in an \
             accumulator with no ceiling.",
            self.mine_unbounded_oracle_bounded
        );
        let floor = c.min_ratio_fraction * self.total as f64;
        assert!(
            n as f64 >= floor,
            "{label}: only n={n} of {} draws produced a comparable ratio, below \
             the anti-vacuity floor of {floor:.0}. A ceiling over too few samples \
             is the defect this guard exists to catch, one level up: find out \
             which bucket absorbed them (census above) before touching this floor.",
            self.total
        );
        assert!(
            self.worst_steps_when_oracle_exact <= c.max_steps_when_oracle_exact,
            "{label}: on the {} draws the oracle proved EXACT, our widest \
             enclosure was {} representable steps; the contract allows {}. This \
             is the class a width ratio cannot score, so it is scored absolutely.",
            self.oracle_exact,
            self.worst_steps_when_oracle_exact,
            c.max_steps_when_oracle_exact
        );
        assert!(
            worst <= c.max_ratio,
            "TIGHTNESS CEILING EXCEEDED for {label}: worst width ratio {worst} vs \
             oracle exceeds {} over n={n} samples. The enclosure got wider; find \
             out why before touching this number, and if the new width is right, \
             re-derive the ceiling rather than restoring the old one.",
            c.max_ratio
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
