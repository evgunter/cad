---
id: assemble-gather-is-the-cost-after-the-census-filter
kind: issue
title: assemble's product gather is the remaining cost after the census pre-filter - 162 ms at 641 solids, n^1.3
status: open
opened: 2026-09-13
priority: P2
cost: H
---


## The finding

Measured by PERF-12 (release, 4 vCPU under the build slot, medians of
3) on the corpus heat sink driven to N fins, N+1 solids, after the
census took the BVH as its pre-filter:

| fins | solids/faces | tier-3′ gate over the aggregate | `assemble` | remainder (the gather) |
|---|---|---:|---:|---:|
| 10 | 11/91 | 1.3 ms | 3.0 ms | 1.7 ms |
| 40 | 41/271 | 3.1 ms | 10.3 ms | 7.2 ms |
| 160 | 161/991 | 9.0 ms | 35.0 ms | 26 ms |
| 640 | 641/3871 | 34.5 ms | 196 ms | 162 ms |

The gate is now ~linear (160 → 640 fins: 3.8× for 4× the entities);
`assemble` is not, and the term that grew is what `assemble` runs
before the gate — `product_recorded` (`crates/editor-core/src/product.rs`)
inside `assemble` (`crates/editor-core/src/assembly.rs`): 26 ms → 162 ms
is 6.2× for 4×, n^1.3. The original finding measured that gather at 23 ms
for 161 solids and set it aside as the linear part; at 641 it is 82% of
`assemble`. DOCM/LIB territory (`product.rs`); not examined further by
PERF-12, whose fence was the census.

## What a fix is

Profile the gather at 640 fins (the instance walk, the name minting, the
contact-record carry) and find the superlinear step — the arena-scan
shape of `work/perf/plan.md` §2.1 is the first suspect.
