---
id: the-three-dual-pins-of-the-place-unit-were-never-shown-able-to-go-red
kind: issue
title: The three dual-against-f64 pins of the place unit passed on CI and were never run against their mutant
status: open
opened: 2026-11-22
priority: P3
cost: E
refs: [7250]
---


## What

PR 7250 replaced a pin that did not test the unit with three dual-against-`f64` pins at sites a dual reaches: `spline_incidence/rows.rs` `at_a_dual_a_point_reads_what_it_reads_at_f64_across_the_band`, `ring_path/spline_wall_rows.rs` `at_a_dual_the_walls_paths_are_the_f64_ones_across_the_band`, `spline_region_rows.rs` `at_a_dual_the_carrier_walk_is_the_f64_one_across_the_band`. Each was argued able to go red by the delta re-review, from its assertions. None was run against a mutant: the machine's build slot was held for hours on 2026-11-21 across the implementer's two passes and the re-review (three agents queued for it; none was granted).

## Owed

In `crates/geom-core/src/dual.rs`, change `view`'s closure to `|d: Self| d.value + d.deriv * T::from_f64(1.0e-3)` (uncommitted), then `cargo test -p geom-brep --lib at_a_dual_a_point_reads` and `cargo test -p topo --lib at_a_dual_the`. Each of the three must go red; a pin that stays green is rewritten until it does. The re-review notes the fixtures use a uniform unit tangent (a fold applied everywhere is nearly a rigid shift) and that the carrier-walk pin rebuilds `read_capped`'s plumbing in the test rather than calling the door. Easy for whichever lane next holds a warm `geom-brep`/`topo` target.
