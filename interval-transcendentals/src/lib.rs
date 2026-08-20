//! # interval-transcendentals
//!
//! Rigorous interval transcendentals over [`libm`], C-free and
//! MIT OR Apache-2.0, with PROVEN outward error pads — the in-house
//! candidate replacement for `inari`'s gmp/MPFR-backed transcendentals
//! (DESIGN.md "Tabled (far future)"; motivation: inari's `gmp` feature
//! drags LGPL-3.0+ transitive deps, needs an AVX+FMA floor for its
//! directed-rounding asm, and upstream is dormant).
//!
//! **Scope is the kernel's inventoried surface** (docs/inventory.md):
//! `sin`/`cos`/`sin_cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sqrt`,
//! `powi`, π-family constants, plus the exact endpoint ops and 1-ulp-padded
//! arithmetic an adoptable interval scalar needs — **and the two interval
//! SET operations, `hull` and `intersection`**, which are not on the
//! `Real` trait and are inventoried separately for that reason. No
//! transcendental arrives without a call site; the census, and the list
//! of functions deliberately not built, are in `docs/inventory.md`.
//!
//! ## Soundness contract
//!
//! Every operation returns an enclosure of the TRUE real image of its
//! input enclosures (containment, certified against inari-with-MPFR as
//! oracle in `tests/`), with the IEEE 1788 decoration as the poison
//! channel: decoration below `Def` must never decide topology (the
//! kernel's M0 ruling).
//!
//! **`Trv` on the wire is not always a domain violation.** Three
//! distinct things lower a decoration to `Trv`, and a consumer reading
//! the channel has to tell them apart:
//! 1. a silent DOMAIN CLAMP (`sqrt` of a zero-straddling box, `asin` /
//!    `acos` past ±1, `tan` where a pole is possible, division by a
//!    zero-touching divisor) — the class this contract is about, and the
//!    reason the clamp can never decide anything;
//! 2. `atan2` at a box containing the ORIGIN, where the function is
//!    genuinely undefined at a point of its own input;
//! 3. **[`DInterval::intersection`], on EVERY input** — an IEEE 1788
//!    formality, not a violation record: the standard gives set
//!    operations no functional meaning, so nothing stronger than `Trv`
//!    may be asserted about one. A result of `[3, 4]` from two clean
//!    operands carries `Trv` and is perfectly trustworthy.
//!
//! (3) is the one that surprises, and it is why it is named here rather
//! than only in `docs/semantics-diffs.md` §D7, where the reasoning
//! lives: a consumer that started consulting the decoration inside
//! `lo()`/`hi()` would poison every intersection result, and that
//! consumer's author is reading this header, not the divergences file.
//! The argument is not restated — §D7 remains its one home.
//!
//! Pads are derived, not guessed: `docs/derivations.md` proves each
//! function pad conservative from (a) the correctly-rounded ops 1-step
//! neighbor lemma and (b) libm's CI-enforced BIT-DISTANCE bounds (1 for
//! the sin family, 2 for atan2) via Lemma P3 (k bit-steps from the
//! correctly rounded reference need k+1 outward steps), backed by the
//! differential harness.
//!
//! ## Honest domain contract (big arguments)
//!
//! Endpoint values of `sin`/`cos`/`tan` are accurate for ALL finite
//! arguments (libm uses full Payne–Hanek reduction). What degrades for
//! huge inputs is *extremum/pole localization*: the conservative grid
//! test loses the ability to prove absence once `|x| ≳ 4·10^15` (π-grid
//! quotient enclosure wider than 1). Consequences, by design:
//! - `sin`/`cos`: widen toward the trivial enclosure `[-1, 1]` — sound,
//!   maximally loose, decoration untouched;
//! - `tan`: returns the whole line with decoration `Trv` (a loud refusal:
//!   pole-freedom is no longer provable).
//!
//! Nothing ever returns a thin wrong interval.
//!
//! ## Tightness (a contract, enforced in both tiers)
//!
//! vs inari (correctly rounded): arithmetic and `sqrt` ≤ 1 ulp per
//! inexact endpoint; transcendentals ≤ 4 ulp per endpoint plus
//! conservative extremum capture; `atan2` boxes containing the origin
//! return the full-range hull (inari may be tighter there). Looseness is
//! acceptable by contract; unsoundness is not.
//!
//! Two things compute with those numbers, because a bound nothing
//! computes with is one careless "let us pad a bit more to be safe" from
//! silently widening every enclosure in the kernel with all lanes green:
//! - `tests/pad_contract.rs` bounds each PAD from above against the
//!   backend's own value, in the oracle-free tier the kernel's CI runs,
//!   so a raised `PAD_ULPS` goes red immediately;
//! - `Tightness`' ceiling in `tests/certify.rs` bounds the whole
//!   ENCLOSURE against the correctly-rounded oracle, which catches a
//!   widening that does not come from a pad.
//!
//! `powi` and the huge-magnitude window carry no ceiling, for reasons
//! written at those call sites.

#![forbid(unsafe_code)]

mod algebraic;
mod arith;
mod consts;
mod interval;
mod invtrig;
mod ops;
mod round;
mod trig;

pub use consts::{frac_pi_2, pi, tau};
pub use interval::{DInterval, Decoration};
