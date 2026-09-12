//! # interval-transcendentals
//!
//! Rigorous interval transcendentals over [`libm`], C-free and
//! MIT OR Apache-2.0, with PROVEN outward error pads — the in-house
//! candidate replacement for `inari`'s gmp/MPFR-backed transcendentals
//! (DESIGN.md "Tabled (far future)"; motivation: inari's `gmp` feature
//! drags LGPL-3.0+ transitive deps, needs an AVX+FMA floor for its
//! directed-rounding asm, and upstream is dormant).
//!
//! **Scope: the public surface is exactly these thirty-one items, and
//! nothing else.**
//! - *transcendental and algebraic:* `sin`, `cos`, `sin_cos`, `tan`,
//!   `asin`, `acos`, `atan`, `atan2`, `sqrt`, `powi`;
//! - *constants:* `pi`, `tau`, `frac_pi_2`;
//! - *endpoint-exact:* `abs`, `min_i`, `max_i`, `floor`, and the
//!   1-ulp-padded operator impls `+ − × ÷ neg`;
//! - *set operations:* `hull`, `intersection`;
//! - *the type itself:* `point`, `from_bounds`, `empty`, `entire`,
//!   `nai`, `lo`, `hi`, `decoration`, `is_empty`, `is_nai`, `contains`,
//!   `with_dec_capped`.
//!
//! That list is checkable — `git grep 'pub fn' src/` returns exactly the
//! non-operator entries above — and it is meant to be: a `pub fn` that
//! is not on it means either the list is wrong or the function should
//! not exist.
//!
//! The list is not the same claim as *"everything here has a caller"*,
//! and that second claim is **false today**: `intersection` has no
//! production call site anywhere in the tree
//! (`docs/inventory.md` records it as `none today`). It is kept
//! deliberately — `docs/semantics-diffs.md` §D7 defines `hull`'s single
//! divergence from IEEE 1788 by contrast with it, so deleting it would
//! delete what a live argument points at — and that is a reason, not an
//! exemption. Which functions the kernel actually consumes, and which
//! are deliberately not built at all, is the census in
//! `docs/inventory.md`.
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
//! 3. **[`DInterval::intersection`], on EVERY input** — see
//!    `docs/semantics-diffs.md` §D7 (titled for `hull`, whose divergence
//!    it is; `intersection` is the 1788-conforming case it is defined
//!    against) for why. A result of `[3, 4]` from two clean operands
//!    carries `Trv` and is perfectly trustworthy.
//!
//! (3) is listed here, and only listed, because a consumer that started
//! consulting the decoration inside `lo()`/`hi()` would poison every
//! intersection result — and that consumer's author is reading this
//! header, not the divergences file. What they need from here is that
//! the case EXISTS; why it is right is §D7's, and is not repeated.
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
//! test loses the ability to prove absence entirely once `|x| ≳ 2^52 ≈
//! 4·10^15` (π-grid quotient enclosure wider than 1), and loses it on
//! SOME inputs from about `|x| ≈ 2^32` — the two thresholds are seven
//! binades apart and `consts::grid_possibly_hits` records both, because
//! the harness's tightness ceilings depend on the earlier one.
//! Consequences, by design:
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
//! Two accumulators carry no ceiling. The huge-magnitude window's is
//! complete — its looseness is documented and unbounded by design. **
//! `powi`'s is a deferral, not an unguardable**: something downstream
//! does compute with that width (`crates/geom-core`'s
//! `powi_f64_lane_is_contained_by_the_padded_enclosure`), and an
//! exponent-dependent ceiling is derivable and not derived. **That is
//! owed work, not a contract** — do not read the absence of a ceiling
//! here as a statement that none exists.

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
