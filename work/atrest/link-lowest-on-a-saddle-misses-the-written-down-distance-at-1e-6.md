---
id: link-lowest-on-a-saddle-misses-the-written-down-distance-at-1e-6
kind: issue
title: link: lowest_on over a saddle reads a distance past the written-down one at ε 1e-6 (fuzz seed 0x5699dbe74052c908)
status: open
opened: 2026-10-29
priority: P2
cost: M
---


Found by PR 5993's CI (`test (kernel)`, the `eps 1e-6` row), on a fuzz
seed of that run; it reproduces on its own:

```
CAD_FUZZ_SEED=0x5699dbe74052c908 CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-6 \
  cargo nextest run -p topo -E 'test(offset_and_lowest_on_meet_the_written_down_distance)'
```

`crates/topo/src/link/oracle_rows.rs:197`
(`offset_and_lowest_on_meet_the_written_down_distance`): "saddle: lowest
on `(-0.2489, 0.1747, -0.9780)`–`(0.8229, -0.9041, 0.8776)` reads
`Ok(-0.24908527011923037)`, not `-0.24890389375700672`", a miss of
`1.8e-4`, far past ε. `link` reads only `geom` and `geom_core`, which
PR 5993 does not touch, so the seed finds it on main as well. The seed
is drawn per run, so the row is green on most runs.

Moved from `work/issues/` to `work/atrest/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## A seed that misses at the default ε (2026-11-06, ROUND orchestrator)

PR 6545's CI (`test (kernel)`, run 42186031508, the default-ε step, on a
diff that touches only `crates/profile`) failed this fuzzer on seed
`0x3e00588bb053d934`: "saddle: lowest on `(-0.48659, -0.02439, 0.23583)`–
`(0.23401, -0.17749, -0.64727)` reads `Ok(-0.17752512685757127)`, not
`-0.1774917394754434`", a miss of 3.3e-5 at ε = 1e-9. So the class is
not confined to the 1e-6 row, and it can red any PR's kernel shard on a
random seed. Reproduce:

```
CAD_FUZZ_SEED=0x3e00588bb053d934 CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo -E 'test(offset_and_lowest_on_meet_the_written_down_distance)'
```
