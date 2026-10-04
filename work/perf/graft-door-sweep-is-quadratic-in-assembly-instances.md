---
id: graft-door-sweep-is-quadratic-in-assembly-instances
kind: issue
title: the graft door's tier-1 sweep makes an N-instance assembly quadratic in a debug-asserting build
status: open
opened: 2026-10-04
priority: P3
cost: E
---


## What

`topo::graft_disjoint_all_keyed` (`crates/topo/src/instance.rs`) declares
D1's door postcondition (`assert_euler_postcondition`), which validates the
whole destination at tier 1. A caller that builds an assembly one instance at a
time (`step_import::import_step`, `editor_core`'s product and pattern
lowerings) grafts N times into a growing body, so the sweeps total O(N²). The
workspace's `[profile.release]` keeps `debug-assertions = true` until
pre-publish, so release builds pay it too.

Measured on one box, release with `debug-assertions`, grafting N copies of the
12-edge `declined_cube` into one body (a scratch timing, not a committed row):

| N | transplant only | staged door body (no sweep) | the door (with sweep) |
|---|---|---|---|
| 200 | 2.8 ms | 4.8 ms | 87 ms |
| 800 | 8.4 ms | 18.6 ms | 1 497 ms |
| 3200 | 35 ms | 72 ms | 32 180 ms |

The staging itself is linear (two transplants of the source). The quadratic
term is the sweep. The step-import suite's assemblies hold at most 14 faces at
graft time, so the suite does not feel it.

## Shape of a fix

The graft's arena shift is exactly the source's arena counts. Tier 1 of the
destination before the graft plus tier 1 of the source implies tier 1 after,
because the two share no key. A sweep restricted to the transplanted entities,
or one over the source before the transplant, would make the door's sweep
O(src). Whether a door may sweep less than the whole body is D1's
once-per-door ruling (`work/perf/d1-per-op-tier1-sweep-price.md`), so a change
here goes back through that ruling.
