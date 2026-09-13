---
id: assemble-aggregate-census-is-quadratic-in-solids
kind: issue
title: assemble's tier-3 census over the aggregate body is quadratic in solids - 1.3 s at 161
status: open
opened: 2026-09-10
parent: PERF-12
---

## The finding

Measured by the PERF kernel lane (`perf/explore-kernel`; release, 4
vCPU). The corpus heat sink driven to N fins, so N+1 solids:

| fins | solids/faces | evaluate | gather | `run_checks` | `assemble` | tier-3′ over the aggregate |
|---|---|---:|---:|---:|---:|---:|
| 10 | 11/91 | 7–9 ms | 1.5 ms | 1.7 ms | 11–12 ms | 9–11 ms |
| 40 | 41/271 | 4–10 ms | 4–6 ms | 4.4 ms | 80–89 ms | 80 ms |
| 160 | 161/991 | 9–12 ms | 23 ms | 25–33 ms | **1.3–1.4 s** | **1.2–1.4 s** |

40 → 160 fins is 4× the size and 15.3× the time (n^1.96). The whole of
`assemble` is the tier-3′ at-rest census over the aggregate body
(`crates/editor-core/src/product.rs:650`; the per-source gate at `:604`
runs only when a source has more than one solid). The gather is 23 ms
at 161 solids in release — the 250–372 ms the closed LIB issue quoted
was the dev profile.

## What a fix is

"Do this faster": the census asks every face pair across solids a
contact question; `run_checks` already answers the linear-cost part.
Either the aggregate census takes the existing `crates/bvh` as a
pre-filter under the conservative-superset contract (D9: it may prune
only pairs the exact predicate would reject, and owes a differential
scenario that is not built from axis-aligned bricks), or the door is
documented as the quadratic one and consumers are pointed at
`run_checks`. Measure which pairs the census actually examines before
choosing. DOCM/LIB territory (`product.rs`), announced.
