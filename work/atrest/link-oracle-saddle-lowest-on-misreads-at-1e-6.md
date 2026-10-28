---
id: link-oracle-saddle-lowest-on-misreads-at-1e-6
kind: issue
title: link's offset_and_lowest_on_meet_the_written_down_distance fuzzer finds a saddle whose lowest-on reads 2.8e-4 off at ε 1e-6
status: open
opened: 2026-10-28
priority: P3
cost: M
---

Found by the GREEN latency lane on a measurement-only PR run (#5953, run
40731565431, job 122970207902, `test (kernel)`, TIER=all extra-ε step).
The branch changes CI only, not `crates/topo`. `topo::link` is ATREST's
(PR 5773), like the sibling `a-ridge-segment-at-contact-escalates-the-touch-plane-at-1e-6`.

## What

`crates/topo/src/link/oracle_rows.rs`
`offset_and_lowest_on_meet_the_written_down_distance` (a seeded fuzzer,
assertion at about line 247) fails at ε = 1e-6:

```
link_oracle: 1 wrong; reproduce with CAD_FUZZ_SEED=0x370a9acaa1355477 CAD_FUZZ_EFFORT=1:
saddle: lowest on Point3 { x: 0.2049934976570722, y: -0.6140123628177865, z: 0.18795387345282233 }–Point3 { x: -0.14203717144214023, y: 0.9411784852774714, z: -0.9009362709508273 } reads Ok(-0.142313290562546), not -0.14203717144214023
```

Replay:
`CAD_FUZZ_SEED=0x370a9acaa1355477 CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p topo --lib offset_and_lowest_on_meet_the_written_down_distance`.

- The same tree passed this row at 1e-6 in four other runs that day
  (#5941), so the counterexample is seed-dependent.
- The miss is 2.8e-4 against the written-down distance. That is far
  outside an ε of 1e-6, so this is not a tolerance-scaling slip.

## Wanted

Pin the seed as a deterministic row beside the fix, then fix either the
`lowest on` reading on a saddle or the oracle's written-down value.
