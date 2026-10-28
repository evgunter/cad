---
id: a-loft-station-that-moves-nothing-refuses-only-at-evaluation
kind: issue
title: A loft station that moves its section by nothing (slide 0) refuses only at evaluation
status: dispatched
opened: 2026-10-26
priority: P3
cost: E
parent: transform-retires-into-a-placement
---

The edit door refuses a loft section whose station is empty (`LoftFault::EmptyStation`, `check_loft_shape` in `crates/editor-core/src/edit.rs`): its data alone stands it in the first section's plane. A station whose moves carry the plane nowhere *by value* is degenerate the same way: a `StationMove::Slide { by: 0 }`, or spins that only turn the plane in itself. The door does not compose the moves, so it admits that loft, and what refuses is the evaluation (`station_plane` and the skin in `crates/editor-core/src/eval/wire.rs`, `wire_loft`). Nothing pins that refusal.

**Done when** a station whose composed plane coincides with the first section's is refused under the current values. The door could compose the moves, as `check_loft_shape` already reads the degree under the current values; or evaluation could refuse it with a typed fault that a row pins. Raised by the review of PR 5748.
