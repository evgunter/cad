# interval-transcendentals

Rigorous interval transcendentals over pure-Rust [`libm`], with **proven
outward error pads** — C-free, `MIT OR Apache-2.0`, `forbid(unsafe_code)`,
no rounding-mode fiddling, no CPU floor.

This is the in-house candidate for the kernel's DESIGN.md tabled item
*"In-house rigorous interval transcendentals"*: a path off `inari`'s
`gmp` feature (LGPL-3.0+ transitive deps `gmp-mpfr-sys`/`rug`, AVX+FMA
inline-asm floor, dormant upstream). **The kernel does not use this crate
yet** — adoption is a separate ratified decision; this crate plus its
certification evidence is the deliverable.

- Standalone cargo project, deliberately excluded from the kernel
  workspace (own `[workspace]` table, like `demos/`).
- Scope: exactly the kernel's inventoried interval-lane surface —
  `docs/inventory.md`.
- Pad proofs: `docs/derivations.md` (neighbor-step lemmas; `PAD_ULPS = 4`
  from libm's CI bit-distance bounds — 1 for the sin family, 2 for atan2 —
  via Lemma P3: k bit-steps from the correctly rounded reference need
  k+1 outward steps; margins 2 and 1 respectively).
- Divergences from inari, all deliberate: `docs/semantics-diffs.md`.

## Certification

`cargo test --release` runs the differential harness against
**inari-with-gmp as dev-dependency oracle** (never shipped; consumers of
this crate inherit zero LGPL obligations): millions of seed-pinned
property cases per run asserting *oracle ⊆ ours* (the oracle's correctly
rounded enclosure contains truth), decoration soundness, empty/NaI
taxonomy agreement, plus edge sweeps (signed zeros, subnormals,
extremum-straddling, huge arguments, poison propagation). Tightness
ratios are printed per function (`--nocapture`).

`--features oracle-computable` adds a second, fully independent oracle
(Evan's `computable` exact-real library, local path dep) on a targeted
sample set.

The oracle needs the repo's x86-64-v3 floor (inherited from the repo-root
`.cargo/config.toml` via cargo's hierarchical config discovery); the
crate itself does not.

## Big-argument contract (honest refusal)

Endpoint *values* are accurate for all finite arguments (libm does full
Payne–Hanek reduction). Extremum/pole *localization* uses a conservative
grid test that loses proving power for `|x| ≳ 4·10^15`: `sin`/`cos`
degrade to `[-1, 1]` (sound, loose), `tan` returns the whole line with
decoration `Trv` (loud refusal). Nothing returns a thin wrong interval.
