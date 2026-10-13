---
id: the-census-curved-span-decision-escalates-near-a-fast-end
kind: issue
title: The census's curved span decision escalates clearly decided points near a fast end, and gates numerics on is_poison
status: closed
branch: fuse/the-localization-core
pr: 4980
closed: 2026-10-13
opened: 2026-10-13
priority: P3
cost: E
---


## The finding

From the PR 4946 round-2 re-review. Both parts fail loud; neither is a wrong read.

- **The span decision escalates where the side check would decide.** In `crates/topo/src/census/curved.rs`, `clear_of_end` first decides `m·lo` (`pm_census_ve_curved_span`). When that lands in its grey band it escalates, rather than falling through to the side check (`m·hi`, `pm_census_ve_curved_side`) and the chord check that follow.
  - A point is therefore refused if it sits between about `(hi/v)·zero` and `(hi/v)·escalate` from a fast end, even though its side and its distance from the end are both clearly decided.
  - Measured on a 1 × 0.05 ellipse at its π/2 end: points ±20ε to ±200ε along the curve escalate, inside and outside, for both vertex-on-edge and edge-edge.
  - The rows pinned in PR 4946 (15ε, and 30ε on 1 × 0.02) sit just under this window.
- **`Real::is_poison` is used as a numeric gate.** Its own doc (`geom-core/src/real.rs`, `is_poison`) says it discriminates structure and "never replaces" a `Decide` poison arm. `curved.rs` uses it as a gate in `read`'s sup check, in the floor refusal (through the `x·0` / `(1/x)·0` idiom) and in `pieces_apart`'s `!second.is_poison()`.
  - At `Interval`, `[a, +∞]·[0, 0] = [0, 0]`, so the idiom would not refuse an unbounded floor. No valid carrier produces one today.
  - `polish_pair` breaks on a poison step, while `polish_foot` relies on `select_le_zero` swallowing a NaN. These are two spellings of "a singular step is dropped".

## What it needs

- Let a grey span decision fall through to the side and chord checks.
- Make the poison gates decisions (or typed refusals) that hold at every scalar, and spell the dropped singular step once.

## 2026-10-13, closed: PR 4980

A grey `m·lo` falls through to the side and chord; the 1 × 0.05 window
(±20ε to ±200ε) decides, vertex and crossing alike
(`near_a_fast_end_the_side_decides_where_the_span_reads_in_band`). The
poison gates are decisions; `stops` reads a floor's enclosure at its
lower end (the round-2 `x·0` refusal is gone); one `step_kept` decides
a polish step on the exact-order band.
