---
id: check-rigid-squares-a-column-by-multiplying-two-copies-of-it
kind: issue
title: check_rigid's three unit-column residuals spell a self-product at Sym<Interval>, where the tight square is the spelling
status: open
opened: 2026-09-21
priority: P2
cost: E
---


**Found by DECIDE-1's static census** (the self-dot straddle; the
census is that PR's body). `check_rigid`
(`crates/topo/src/transform.rs`, ~`:244`) builds its three unit-column
rigidity residuals as

```rust
("transform_rigid_col0_unit", l.c0.dot(l.c0) - one),
```

— `Vec3::dot` called with the same vector twice. It is generic over
`T: Decide` and reached at `Sym<Interval>` from `editor-core`'s
placement step (`eval/wire.rs`, `transform_rigid`), so `l.c0` is an
enclosure and a column entry near zero STRADDLES zero. For such an
entry `[-a, b] · [-a, b]` has the spurious lower bound `-ab` where the
tight square `powi(2)` has the exact `0`
(`RingInterval::sqr`'s docs; `interval.rs`'s `powi`, pinned by
`powi_is_tight_across_zero`). `Vec3::norm_squared` is the door that
squares component-wise and already exists — `l.c0.norm_squared() - one`
is the whole change.

**This is NOT the `sqrt`-domain-violation class** the finding lane was
looking for: the consumer is `geom_core::k_stats::decide_flagged`, a
`sign_within`, so a spurious negative lower bound cannot become a
clause-1 `Invalid` here. What it can do is WIDEN a residual that has to
classify `Zero` against the linear band — and this door refuses on
in-band indeterminacy as well as on a definite non-zero, so a widened
residual is a maybe-rigid map that refuses. The widening is second
order in the column entry (a rigid frame's small entries sit at
rounding width), which is why this is filed rather than urgent.

**What is owed:** the swap to `norm_squared()`, with the f64
bit-identity across it pinned the way
`scripts/gates/interval-square-allowlist.sh` gates that class
(`x * x` and `powi(2)` are bit-identical at f64 where `powi` is one
multiplication), and a row that exercises the three residuals at
`Sym<Interval>` on a placed body — **nothing in the measured corpus
covers them today**, and that is reproducible rather than asserted:
`crates/editor-core/tests/decide_1_self_dot_interval.rs` prints, per
replay, the `transform_rigid_*` rows that DECIDED there (a filter over
`m10_8_harness::split`, which keeps every predicate that decided at
all — the blocked table cannot show a predicate that never ran). It
reads `none` on every row of the census, at all three ε and at every
scale up to twice the real study. The readout is R1's, from DECIDE-1's
review probe.
