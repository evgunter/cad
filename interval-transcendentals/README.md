# interval-transcendentals

Rigorous interval transcendentals over pure-Rust [`libm`], with **proven
outward error pads** — C-free, `MIT OR Apache-2.0`, `forbid(unsafe_code)`,
no rounding-mode fiddling, no CPU floor.

This is the kernel's interval backend, and the resolution of the
DESIGN.md tabled item *"In-house rigorous interval transcendentals"*: a
path off `inari`'s `gmp` feature (LGPL-3.0+ transitive deps
`gmp-mpfr-sys`/`rug`, AVX+FMA inline-asm floor, dormant upstream).
`geom-core`'s `interval` feature depends on this crate, and no kernel
build in any configuration links C or LGPL code.

- Standalone cargo project, deliberately excluded from the kernel
  workspace (own `[workspace]` table, like `demos/`) so that its
  gmp-backed certification oracle can never reach a kernel build.
- Scope: exactly the kernel's inventoried interval-lane surface —
  `docs/inventory.md`.
- Pad proofs: `docs/derivations.md`.
- Divergences from inari, all deliberate: `docs/semantics-diffs.md`.

## Certification

Two tiers, split so that the cheap one can run in kernel CI.

**`cargo test`** (no features) needs **no oracle and no C toolchain** —
unit tests, the `edges.rs` sweep (signed zeros, subnormals,
extremum-straddling, huge arguments, poison propagation),
`pad_contract.rs`'s upper bound on every pad, and
`review_fuzz_exact.rs`'s exact-rational fuzz of `÷`, `×` and `sqrt`,
which compares against exact `u128` rational arithmetic. This is the
tier the kernel's CI runs.

**What that tier catches, exactly.** A pad WIDENED — on any operation —
goes red here, because `pad_contract.rs` bounds each endpoint's distance
from the backend's own value by the derived pad. A pad DROPPED goes red
here for `÷ × sqrt` only, where the exact-rational fuzz can compute the
truth itself.

Two families are outside that, in the same direction and for different
reasons. **`+ −`:** their witness (TwoSum) has no validity floor to get
wrong — its error term is representable for all finite doubles with no
underflow proviso (`docs/derivations.md` §1 Lemma P0), so there is no
lying-witness failure mode to fuzz — but their *containment* is
unguarded here: mutating `add_lo`/`add_hi` to bare round-to-nearest
leaves this whole tier green. The u128 comparator the fuzz is built on
cannot serve them, since aligning `2^1023` with `2^-1074` needs ~2100
bits. **The seven transcendentals:** their truth needs a multi-precision
reference at all.

Both are the oracle tier's, and it runs on **four paths** —
`interval-transcendentals/src/`, `tests/`, `Cargo.toml`, `Cargo.lock`
(`scripts/ci-filter.py`'s `ORACLE_PATHS`) — not on the whole directory.
`docs/` is deliberately not among them, so a change to the derivation
that sets `PAD_ULPS` does not by itself re-certify; changing the code it
justifies does.

The fuzz lanes draw from the tree's shared harness (`test-utils`, a
dependency-free dev-only crate). Every test here is a counterexample
search — *∀ sampled x, oracle ⊆ ours* — so the seed **varies per run and
is logged unconditionally**, and pinning it would re-certify the same
points forever however often the lane runs. `CAD_FUZZ_SEED=0x…` replays
a run exactly; `CAD_FUZZ_EFFORT` scales depth (the shipped level is a
smoke sweep across the three operations).

**`cargo test --release --features oracle-inari`** adds `certify.rs`, the
differential harness against **inari-with-gmp as a dev-dependency oracle**
(never shipped; consumers inherit zero LGPL obligations): property cases
asserting *oracle ⊆ ours* (the oracle's correctly rounded enclosure
contains truth), decoration soundness, and empty/NaI taxonomy agreement.
Tightness ratios are printed per function (`--nocapture`). The oracle is
optional precisely so that the default `cargo test` pulls no C toolchain.

The oracle needs AVX+FMA — inari's rounding primitives are behind
`cfg(all(target_feature = "avx", target_feature = "fma"))` and it raises a
`compile_error!` without them. The repo sets no target-cpu flags
anywhere, deliberately, so the flag is supplied per-invocation, and only
the oracle needs it:

```
RUSTFLAGS="-C target-cpu=x86-64-v3" cargo test --release --features oracle-inari
```

`.github/workflows/ci.yml`'s `oracle-certify` job runs this tier whenever
any of the four `ORACLE_PATHS` changes. Case depth is one env var away —
the job's `CAD_FUZZ_EFFORT` multiplies every count — and depth is cheap,
because the job's cost is dominated by building GMP and MPFR from C
source, not by the cases.

The **local** gate does not cover it: `local-scripts/ci-local.sh` mirrors
the cheap row and has no `oracle-certify` row, so under `gate.sh` — the
merge gate when hosted Actions is unavailable — a dropped transcendental
pad is not caught. Recorded as smell-scan **S127**.

## Big-argument contract (honest refusal)

Endpoint *values* are accurate for all finite arguments (libm does full
Payne–Hanek reduction). Extremum/pole *localization* uses a conservative
grid test that loses proving power entirely for `|x| ≳ 2^52 ≈ 4·10^15`,
and partially from about `|x| ≈ 2^32`: `sin`/`cos` degrade to `[-1, 1]`
(sound, loose), `tan` returns the whole line with decoration `Trv` (loud
refusal). Nothing returns a thin wrong interval.
