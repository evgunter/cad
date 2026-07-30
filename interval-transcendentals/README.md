# interval-transcendentals

Rigorous interval transcendentals over pure-Rust [`libm`], with **proven
outward error pads** — C-free, `MIT OR Apache-2.0`, `forbid(unsafe_code)`,
no rounding-mode fiddling, no CPU floor.

This is the kernel's interval backend, and the resolution of the
DESIGN.md tabled item *"In-house rigorous interval transcendentals"*: a
path off `inari`'s `gmp` feature (LGPL-3.0+ transitive deps
`gmp-mpfr-sys`/`rug`, AVX+FMA inline-asm floor, dormant upstream). Since
M5 PR 1 `geom-core`'s `interval` feature depends on this crate, and no
kernel build in any configuration links C or LGPL code.

- Standalone cargo project, deliberately excluded from the kernel
  workspace (own `[workspace]` table, like `demos/`) so that its
  gmp-backed certification oracle can never reach a kernel build.
- Scope: exactly the kernel's inventoried interval-lane surface —
  `docs/inventory.md`.
- Pad proofs: `docs/derivations.md` (neighbor-step lemmas; `PAD_ULPS = 4`
  from libm's CI bit-distance bounds — 1 for the sin family, 2 for atan2 —
  via Lemma P3: k bit-steps from the correctly rounded reference need
  k+1 outward steps; margins 2 and 1 respectively).
- Divergences from inari, all deliberate: `docs/semantics-diffs.md`.

## Certification

Two tiers, split so that the cheap one can run in kernel CI:

**`cargo test`** (no features) needs **no oracle and no C toolchain** —
unit tests, the
`edges.rs` sweep (signed zeros, subnormals, extremum-straddling, huge
arguments, poison propagation), and `review_fuzz_div.rs`'s exact-rational
division fuzz, which needs no oracle at all because it compares against
exact `u128` rational arithmetic. This is the tier the kernel's CI runs,
so a dropped pad is caught by the same pipeline that gates the kernel.

**`cargo test --release --features oracle-inari`** adds `certify.rs`, the
differential harness against **inari-with-gmp as a dev-dependency oracle**
(never shipped; consumers inherit zero LGPL obligations): millions of
seed-pinned property cases per run asserting *oracle ⊆ ours* (the
oracle's correctly rounded enclosure contains truth), decoration
soundness, and empty/NaI taxonomy agreement. Tightness ratios are printed
per function (`--nocapture`). The oracle is optional precisely so that
the default `cargo test` pulls no C toolchain; run this tier by hand
whenever the rounding layer or the pads change.

The oracle needs the repo's x86-64-v3 floor (inherited from the repo-root
`.cargo/config.toml` via cargo's hierarchical config discovery); the
crate itself does not.

## Big-argument contract (honest refusal)

Endpoint *values* are accurate for all finite arguments (libm does full
Payne–Hanek reduction). Extremum/pole *localization* uses a conservative
grid test that loses proving power for `|x| ≳ 4·10^15`: `sin`/`cos`
degrade to `[-1, 1]` (sound, loose), `tan` returns the whole line with
decoration `Trv` (loud refusal). Nothing returns a thin wrong interval.
