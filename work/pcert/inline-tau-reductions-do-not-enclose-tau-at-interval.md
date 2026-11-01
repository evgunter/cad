---
id: inline-tau-reductions-do-not-enclose-tau-at-interval
kind: issue
title: inline [0, τ) reductions built on T::from_f64(TAU) do not enclose τ at Interval
status: open
opened: 2026-10-14
priority: P3
cost: E
refs: [the-period-jump-has-no-ruling]
---


Found by PR 5080's review (R1, a style finding).

**The defect.** Several open-coded `[0, τ)` reductions build their
period as `T::from_f64(core::f64::consts::TAU)`. At `Interval` that is
the point enclosure of `fl(τ)`, which does not contain τ. A reduction
built on it does not enclose the true reduced value: `T::tau()` is the
enclosing constant. The `Real::reduce_periodic` contract ("contains the
true reduced value whenever the inputs enclose theirs") only holds with
an enclosing period.

**Sites:**
- `crates/topo/src/boolean/rim_wedge.rs`:
  - `departures` (`let tau = …`, about :315);
  - `at` (about :474);
  - `locus_param` (about :532);
  - `reach` (about :608);
  - `crossings`' strictly-inside candidate read (about :870).
- `crates/editor-core/src/names/emit_topo.rs`:
  - `Track::of_edge`'s `period` (`Some(T::from_f64(std::f64::consts::TAU))`, about :3160);
  - its readers `Track::offset` (about :3239) and `stretches`' `fwd` (about :3271).

**Territory.** `rim_wedge.rs` is CLEAVE's and HONE's; `emit_topo.rs` is
REFS's and WIRE's. It is filed on PCERT because the defect is the
period's enclosure, PCERT's ground. The fix is `T::tau()` at each site,
plus an Interval row that the point enclosure fails.
