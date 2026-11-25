---
id: main-red-endpoint-census-reads-ruling-rs-at-nine-since-the-cone-ruled-side
kind: issue
title: Main red (slow set): certified_endpoint_census reads pair_section/ruling.rs at (9, 9) where the roster pins (3, 3), since PR 7357
<<<<<<< HEAD
status: dispatched
opened: 2026-11-23
priority: P1
cost: E
refs: [7357]
branch: sectcone/endpoint-census-ruling
=======
status: closed
opened: 2026-11-23
priority: P1
cost: E
refs: [7357, 7452]
closed: 2026-11-24
>>>>>>> origin/main
---


Found by CHARTTAIL's `charttail/speed-census` lane (2026-11-23), running the census for its own `speed.rs` line on main `ba685969fc`.

## What

`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` (slow set) reads `crates/geom-brep/src/pair_section/ruling.rs` at `(9, 9)`; its roster pins `(3, 3)`. The six new reads arrived with PR 7357 (`sectcone/cone-ruled-side`, merge `656aa486df`; the file holds 3 on its first parent, 9 on the merge). They are all in `coverage` (`h.lo() > 0.0 || h.hi() < 0.0`, `hull(disc).hi() < 0.0`, and the `n`, `d`, `reach`, `speed` reads), which asks `is_certified()`, so every one counts as asking.

## Owed

Read the six against the ask they sit behind, move the roster's `ruling.rs` line to the tree's count, and write its note. `coverage`'s two sign tests on endpoints (`h.lo() > 0.0 || h.hi() < 0.0`, `hull(disc).hi() < 0.0`) are decisions on a margin's sign; whether they belong in the funnel (`decide`, `decide_nonzero`) is part of the disposition.

    cargo nextest run -p geom-core --test all --ignore-default-filter -E 'test(=certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned)'

## Closed

The roster's `ruling.rs` line already reads the tree's `(9, 9)`: `census: four files re-counted on main` (PR 7452, `55b5e2433b`) moved it before this row was dispatched. Of the nine reads, three predate PR 7357 (`clear`'s gap 2 and `Δ`'s high end), not six of them as the What section says. This row's branch, `sectcone/endpoint-census-ruling`, writes the note for all nine. The two sign tests stay off the funnel. Each is a certified exclusion of zero that the walk states as a proof: `Δ < 0` shows a ruling misses the partner, and `apart` shows the root's divisor is off zero. Neither is a decision at the band, and a band would only make them refuse more. `coverage` runs only under `debug_assertions`, so routing them through `decide` would also record decisions that release builds never make.
