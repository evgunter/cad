//! # interval-transcendentals
//!
//! Rigorous interval transcendentals over [`libm`], C-free and
//! MIT OR Apache-2.0, with PROVEN outward error pads — the in-house
//! candidate replacement for `inari`'s gmp/MPFR-backed transcendentals
//! (DESIGN.md "Tabled (far future)"; motivation: inari's `gmp` feature
//! drags LGPL-3.0+ transitive deps, needs an AVX+FMA floor for its
//! directed-rounding asm, and upstream is dormant).
//!
//! **Scope is the kernel's inventoried surface only** (docs/inventory.md):
//! `sin`/`cos`/`sin_cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sqrt`,
//! `powi`, π-family constants, plus the exact endpoint ops and 1-ulp-padded
//! arithmetic an adoptable interval scalar needs. Nothing else.
//!
//! ## Soundness contract
//!
//! Every operation returns an enclosure of the TRUE real image of its
//! input enclosures (containment, certified against inari-with-MPFR as
//! oracle in `tests/`), with the IEEE 1788 decoration as the poison
//! channel (decoration below `Def` must never decide topology — the
//! kernel's M0 ruling; silent domain clamps drop the decoration to `Trv`
//! and thereby never decide).
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
//! ## Tightness (documented, measured in the harness)
//!
//! vs inari (correctly rounded): arithmetic ≤ 1 ulp per inexact endpoint;
//! transcendentals ≤ 4 ulp per endpoint plus conservative extremum
//! capture; `atan2` boxes containing the origin return the full-range
//! hull (inari may be tighter there). Looseness is acceptable by
//! contract; unsoundness is not.

#![forbid(unsafe_code)]

mod algebraic;
mod arith;
mod consts;
mod interval;
mod invtrig;
mod ops;
mod round;
mod trig;

// Kani proof harnesses (docs/verification.md). `cfg(kani)` is set only by
// the Kani driver, so this module does not exist in any cargo build.
#[cfg(kani)]
mod verify;

pub use consts::{frac_pi_2, pi, tau};
pub use interval::{DInterval, Decoration};
