//! Shared harness for the tree's randomized falsification sweeps —
//! every workspace in it, including `interval-transcendentals/`, which
//! is its own root.
//!
//! Three things the sweeps each hand-rolled a copy of, in one place: the
//! xorshift64\* generator, the per-run **seed**, and the **effort** dial
//! every iteration count is a multiple of.
//!
//! # Which shape is your test? Read this before choosing a seed
//!
//! Three shapes. **Only one of them wants a varying seed**, and reading
//! "a fuzzer must not fix its seed" as "everything needs a random seed"
//! is how a coverage test becomes flaky.
//!
//! **1. Counterexample search** — *for all sampled `x`, `P(x)`*. The
//! default, and what this harness is for. **Vary the seed.** It is
//! monotone in the safe direction: cutting the count loses detection
//! power but can never make the test fail, so `EFFORT` is safe to turn
//! down and successive runs explore new ground.
//!
//! **2. A witness you can WRITE DOWN** — *at least `K` sampled `x` are
//! of class `C`*, where `C` is concisely constructible. **Do not search
//! at all.** Build the case as a static fixture and assert on it every
//! run. Hunting for something you could construct buys it on ~99% of
//! runs instead of 100%, and makes the sample count carry an obligation
//! that is anti-monotone. `profile`'s enclosing-tangency case was this:
//! it inflated a row 8x to stumble onto something the geometry names
//! directly (rho = R - sigma*tau*r < 0).
//!
//! **3. A witness you CANNOT write down** — same shape, but `C` is not
//! concisely specifiable: "a walk that reaches every op kind and every
//! site shape" is not something you can spell out. **Fix the seed.**
//! Here the seed is a FIXTURE IDENTIFIER — compression for an input too
//! big to write down — not a sampling strategy. It is deterministic, so
//! it cannot flake, and it is cheaper because nothing has to be
//! over-provisioned against bad luck.
//!
//! The condition on 3 (Ev, 2026-08-13): `K` must be large enough — or
//! the simultaneous conditions numerous enough — that the row is **very
//! unlikely to pass by accident on a lucky seed**. `K = 1` against a
//! 1-in-1000 class is the shape to avoid: the one seed that happens to
//! work then carries the whole claim, and nobody can tell a real pass
//! from a fortunate draw. `topo`'s `seqgen` coverage rows clear it
//! comfortably: they must hit every op kind AND every site shape at
//! once, so a seed that satisfies them is a good seed rather than a
//! lucky one.
//!
//! **The trap is mixing 1 and 3 in one test.** A property test with an
//! anti-vacuity floor bolted on is both shapes at once, and its sample
//! count then feeds two obligations of which only one is safe to cut.
//! Either make the floor's witness static (shape 2) or split the test.
//!
//! # Why the seed is not hardcoded
//!
//! Every sweep in this workspace used to open with a literal seed
//! (`Rng(0x9e37_79b9_7f4a_7c15)` and friends). That makes the sweep a
//! *replay corpus*: it explores exactly the same points on every run
//! for the rest of the project's life, so after its first green run it
//! can only ever fail because the code under test changed. Useful — but
//! it is not a fuzzer, and it should not be described or budgeted as
//! one.
//!
//! The seed is therefore drawn fresh per process, and **logged
//! unconditionally** by [`start`] so a run always records what it
//! explored. Pin it with `CAD_FUZZ_SEED` to replay a failure exactly.
//! CI's fuzz job exports one seed for the whole job, so a red run names
//! a single value that reproduces every sweep in it.
//!
//! # Why effort is a dial and not a constant
//!
//! The shipped counts are a **smoke level**: enough to catch a gross
//! regression in the seconds a gated CI job should cost, and no more.
//! Depth is bought deliberately, not paid for on every run — set
//! `CAD_FUZZ_EFFORT=100` to restore roughly the depth these sweeps had
//! before 2026-08-13, or higher to actually hunt.
//!
//! Because the seed varies, low effort per run is not the same as low
//! coverage over time the way it was with fixed seeds: successive runs
//! explore different points. What it does mean is that any single run
//! is a shallow sample, which is the honest description of what a CI
//! smoke sweep is.
//!
//! # Discipline
//!
//! A sweep that finds a real defect has produced a *specific*
//! counterexample. Pin that case as an ordinary deterministic test
//! alongside the fix — the sweep's job is to find it, not to be the
//! regression gate for it afterwards.

// This module PANICS on a malformed dial, deliberately: a typo'd
// `CAD_FUZZ_EFFORT` that silently fell back to the default would report
// a depth the run never ran at, which is the failure this whole harness
// exists to prevent. The workspace's blanket no-panic rule is about
// production code, and nothing on a shipped build path can reach here —
// no production manifest names this crate at all (see the crate docs),
// the same posture the test suites take.
#![allow(clippy::panic)]

use std::sync::OnceLock;

/// `xorshift64*` — one stream, no threads, bit-reproducible from its
/// seed. The single copy; the per-file duplicates it replaces were
/// byte-identical to this.
pub struct Rng(u64);

impl Rng {
    /// A stream from an explicit seed. Zero is remapped: xorshift64\*
    /// is absorbing at zero, so a zero seed would yield a constant
    /// stream and silently turn a sweep into one repeated case.
    pub fn from_seed(seed: u64) -> Self {
        Self(if seed == 0 {
            0x9e37_79b9_7f4a_7c15
        } else {
            seed
        })
    }

    /// The next raw draw. All the other generators are shaped from this.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// Uniform in `[0, 1)` from the top 53 bits.
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9_007_199_254_740_992.0)
    }

    /// Uniform in `[lo, hi)`.
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }

    /// A uniform index in `0..n`. Panics on `n == 0`, like indexing does.
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

fn parse_env<T: std::str::FromStr>(var: &str) -> Option<T> {
    match std::env::var(var) {
        Err(_) => None,
        Ok(raw) => Some(raw.trim().parse().unwrap_or_else(|_| {
            // Fail loud: a typo'd dial that silently fell back to the
            // default would report a depth the run never ran at.
            panic!("{var} is set to {raw:?}, which does not parse")
        })),
    }
}

/// The seed for this process. Random unless `CAD_FUZZ_SEED` pins it.
///
/// Hex with or without a `0x` prefix, or decimal — the value [`start`]
/// prints is accepted back verbatim.
pub fn seed() -> u64 {
    static SEED: OnceLock<u64> = OnceLock::new();
    *SEED.get_or_init(|| {
        if let Ok(raw) = std::env::var("CAD_FUZZ_SEED") {
            let t = raw.trim();
            let parsed = t
                .strip_prefix("0x")
                .or_else(|| t.strip_prefix("0X"))
                .map(|h| u64::from_str_radix(&h.replace('_', ""), 16))
                .unwrap_or_else(|| t.replace('_', "").parse());
            return parsed.unwrap_or_else(|_| {
                panic!("CAD_FUZZ_SEED is set to {raw:?}, which does not parse")
            });
        }
        // No entropy dependency in the kernel's graph for a test dial:
        // process id and the wall clock, mixed with splitmix64. Nextest
        // runs a process per test, so this differs per test unless CI
        // pins one value for the whole job.
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        let mut z = t ^ (u64::from(std::process::id()) << 32);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    })
}

/// The iteration multiplier. Every randomized sweep states its work as
/// a multiple of this, so one dial scales the whole battery.
///
/// Defaults to 1 — the shipped smoke level. `CAD_FUZZ_EFFORT=100` is
/// roughly the depth these sweeps ran at before they were scaled.
pub fn effort() -> u32 {
    static EFFORT: OnceLock<u32> = OnceLock::new();
    *EFFORT.get_or_init(|| {
        let e: u32 = parse_env("CAD_FUZZ_EFFORT").unwrap_or(1);
        assert!(e > 0, "CAD_FUZZ_EFFORT must be at least 1, got {e}");
        e
    })
}

/// `n` scaled by [`effort`], never below `n` and never zero — the one
/// place the multiplication happens, so no sweep can scale to nothing.
pub fn scaled(n: usize) -> usize {
    n.saturating_mul(effort() as usize).max(1)
}

/// Open a sweep: log what it is about to explore, and hand back its
/// stream.
///
/// The log line is UNCONDITIONAL. A sweep that only printed its seed on
/// failure would be unreproducible in the one case that matters most —
/// the intermittent one — and nextest captures per-test output and
/// replays it for failures anyway, so the line costs nothing and is
/// always there when needed. Include [`replay`] in assertion messages
/// too; a reader looking at a failed assert should not have to scroll.
///
/// `label` distinguishes streams within one process, so two sweeps in
/// the same test binary do not explore identical draws.
pub fn start(label: &str) -> Rng {
    let s = seed();
    println!(
        "[fuzz] {label}: seed=0x{s:016x} effort={} — replay with CAD_FUZZ_SEED=0x{s:016x}",
        effort()
    );
    // Mix the label in so streams differ per sweep while one env var
    // still pins them all.
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for b in label.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    Rng::from_seed(s ^ h)
}

/// Open a PINNED stream for a shape-3 coverage claim — a witness you
/// cannot write down (see the taxonomy at the top of this module).
///
/// Deterministic by default, which is the entire point: a coverage
/// claim on a varying seed is a witness search that can flake, and
/// over-provisioning the budget to survive bad luck is paying to keep a
/// fragile design alive. The seed is a FIXTURE IDENTIFIER here.
///
/// Still logged, exactly like [`start`] — a pinned seed nobody can see
/// is the shape this harness exists to remove.
///
/// `CAD_FUZZ_SEED` DOES override it, deliberately: that is how you check
/// the claim is not specific to its pinned seed (a coverage row that
/// only holds for one draw is a row fitted to its seed). The log line
/// says which seed was used and whether it was the pinned one, so a run
/// that explored elsewhere cannot be mistaken for the gate.
pub fn pinned(label: &str, seed: u64) -> Rng {
    let overridden = std::env::var_os("CAD_FUZZ_SEED").is_some();
    let s = if overridden { self::seed() } else { seed };
    println!(
        "[fuzz] {label}: seed=0x{s:016x} effort={} ({})",
        effort(),
        if overridden {
            "OVERRIDDEN by CAD_FUZZ_SEED — exploring, not the pinned gate"
        } else {
            "pinned coverage seed"
        }
    );
    Rng::from_seed(s)
}

/// The exact environment that reproduces this process's draws — put it
/// in assertion messages.
pub fn replay() -> String {
    format!(
        "reproduce with CAD_FUZZ_SEED=0x{:016x} CAD_FUZZ_EFFORT={}",
        seed(),
        effort()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pinned_seed_is_reproducible_and_a_zero_seed_is_not_absorbing() {
        let a: Vec<u64> = (0..8)
            .scan(Rng::from_seed(12345), |r, _| Some(r.next_u64()))
            .collect();
        let b: Vec<u64> = (0..8)
            .scan(Rng::from_seed(12345), |r, _| Some(r.next_u64()))
            .collect();
        assert_eq!(a, b, "the same seed must give the same stream");
        let z: Vec<u64> = (0..8)
            .scan(Rng::from_seed(0), |r, _| Some(r.next_u64()))
            .collect();
        assert!(
            z.windows(2).any(|w| w[0] != w[1]),
            "a zero seed must not collapse to a constant stream"
        );
    }

    #[test]
    fn distinct_labels_draw_distinct_streams() {
        let mut a = start("alpha");
        let mut b = start("beta");
        assert_ne!(
            (0..4).map(|_| a.next_u64()).collect::<Vec<_>>(),
            (0..4).map(|_| b.next_u64()).collect::<Vec<_>>(),
            "two sweeps in one process must not explore identical draws"
        );
    }

    #[test]
    fn scaling_never_reaches_zero() {
        assert!(scaled(0) >= 1, "a sweep must never scale to no work at all");
        assert!(scaled(7) >= 7, "effort must never scale a count DOWN");
    }
}
