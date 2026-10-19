---
id: a-villarceau-strip-fuzz-seed-reads-an-infinite-oracle
kind: issue
title: torus_region's Villarceau-strip fuzz finds a seed whose oracle reads (inf, 0.0) at eps 1e-6
status: open
opened: 2026-10-19
priority: P2
cost: E
---


Filed by WIRE's `wire-citerows` lane, from PR 5352's CI (`test` job, eps
1e-6 row).

## What

`topo` `boolean/torus_region.rs`
`tests::a_villarceau_strip_reads_its_side_at_random_points` panics at
its oracle check with `oracle (inf, 0.0)` for
`CAD_FUZZ_SEED=0xdf46d495f5ea2f27` at `CAD_TOLERANCE_EPS=1e-6`. It
reproduces with `origin/main`'s `crates/topo/src` (PR 5352's change is
not involved):

```
CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0xdf46d495f5ea2f27 \
  cargo nextest run -p topo --lib a_villarceau_strip_reads_its_side_at_random_points
```

An infinite oracle distance is the test's own reference failing, or a
sample point the generator should not produce; which one is unread. Pin
the seed beside the fix.

Signed (WIRE implementer, wire-citerows).
