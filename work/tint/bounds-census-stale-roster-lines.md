---
id: bounds-census-stale-roster-lines
kind: issue
title: bounds_census's roster keeps two stale lines (curves/boxes.rs lifted, surgery.rs boxed_reach) and is red on main
status: open
opened: 2026-11-03
priority: P1
cost: E
---


Found by ONECOPY's frame-readings unit (PR 6463), which updated this census for its own sites.

`crates/geom-core/tests/bounds_census.rs` `every_sole_bracket_bound_door_is_in_the_roster` (slow set, nightly) is red on main:

- **arrived or moved**: `crates/geom/src/curves/boxes.rs` `lifted`. The roster lists it under the subject `fn lifted<T: Bounds>`, but the census now reads the item's subject as its name, `lifted`.
- **listed but no longer present**: `crates/geom/src/curves/boxes.rs` `fn lifted<T: Bounds>`, and `crates/sweep/src/blend/surgery.rs` `boxed_reach`. On main `boxed_reach` is bounded by `SpanLocate`, not `Bounds`.

Fix: re-point `lifted`'s line at its subject as the census reads it, keeping its disposition, and drop `boxed_reach`'s (or record why its new bound still owes a line).
